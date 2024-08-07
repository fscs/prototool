use std::fs;

use std::fs::OpenOptions;

use anyhow::{Context, Result};
use askama::Template;

use arboard::Clipboard;

#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
#[cfg(target_os = "linux")]
use libc::fork;
#[cfg(target_os = "linux")]
use rustix::stdio::{dup2_stdin, dup2_stdout};

use chrono::NaiveDateTime;
use clap::{ArgGroup, Args};
use reqwest::blocking::Client;
use url::Url;

use super::Runnable;
use prototool::protokoll::ProtokollTemplate;

use prototool::post;
use prototool::protokoll::{events, raete, tops};
use prototool::sitzung;

/// Generate a new Protokoll
#[derive(Debug, Args)]
#[clap(group(
            ArgGroup::new("import_export")
                .args(&["to_clipboard", "from_clipboard", "to_pad", "from_pad"]),
        ))]
pub struct GenerateCommand {
    /// Endpoint to fetch Tops from
    #[arg(short = 'U', long, default_value = "https://fscs.hhu.de/")]
    pub endpoint_url: Url,
    /// Under which language the protokoll should be created
    #[arg(short, long, default_value = "de")]
    pub lang: String,
    /// Open the protokoll for editing.  
    #[arg(long, short)]
    pub edit: bool,
    /// Force creation, even if a file already exist
    #[arg(long, short)]
    pub force: bool,
    /// Generate the protokoll into the system clipboard
    #[arg(long)]
    pub to_clipboard: bool,
    /// Load the protokoll content from the system clipboard
    #[arg(long)]
    pub from_clipboard: bool,
    /// Copies the protokolls content into the system clipboard and opens an appropriate
    /// pad url in the webbrowser
    #[arg(long)]
    pub to_pad: bool,
    /// Load the protokoll content from a hedgedoc note
    #[arg(long, value_name = "PAD_URL")]
    pub from_pad: Option<Url>,
}

impl Runnable for GenerateCommand {
    fn run(&self) -> Result<()> {
        let client = Client::new();

        println!("fetching sitzung...");

        let now = chrono::Utc::now()
            .naive_local()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let next_sitzung = sitzung::fetch_sitzung(&self.endpoint_url, &client, &now)?;
        let timestamp = next_sitzung.date;

        if self.from_clipboard {
            return self.create_from_clipboard(&timestamp);
        } else if let Some(pad_url) = &self.from_pad {
            return self.create_from_pad(&client, pad_url, &timestamp);
        }

        let template = self.build_template(&client, &now)?;

        drop(client);

        if self.to_clipboard {
            self.create_in_clipboard(template)
        } else if self.to_pad {
            self.create_in_pad(&timestamp, template)
        } else {
            self.create_locally(&timestamp, template)
        }
    }
}

impl GenerateCommand {
    fn build_template(
        &self,
        client: &Client,
        timestamp: &NaiveDateTime,
    ) -> Result<ProtokollTemplate> {
        println!("fetching tops...");
        let tops = tops::fetch_tops(&self.endpoint_url, client, timestamp)?;

        println!("fetching räte and withdrawals...");
        let persons = raete::fetch_persons(&self.endpoint_url, client, &timestamp.date())?;
        let abmeldungen = raete::fetch_abmeldungen(&self.endpoint_url, &client, &timestamp.date())?;
        let raete = raete::determine_present_räte(&persons, &abmeldungen);

        println!("fetching events...");
        let events = events::fetch_calendar_events(&self.endpoint_url, &client)?;

        return Ok(ProtokollTemplate {
            datetime: timestamp.to_owned(),
            tops,
            raete,
            events,
        });
    }

    fn write_to_file(&self, timestamp: &NaiveDateTime, content: &str) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;
        let path = format!(
            "protokolle/{}/{}-protokoll.md",
            timestamp.format("%Y"),
            timestamp.format("%m-%d"),
        );

        let file_path = post::create_post(&cwd, &self.lang, &path, self.force)?;

        fs::write(&file_path, content)?;

        println!("created Protokoll at '{}'", file_path.to_string_lossy());

        if self.edit {
            post::edit(&file_path)?
        }

        Ok(())
    }

    fn create_locally(&self, timestamp: &NaiveDateTime, template: ProtokollTemplate) -> Result<()> {
        let rendered = template
            .render()
            .context("error while rendering template")?;

        self.write_to_file(timestamp, rendered.as_str())
    }

    #[cfg(not(target_os = "linux"))]
    fn create_in_clipboard(&self, template: ProtokollTemplate) -> Result<()> {
        let mut clipboard = Clipboard::new().context("unable to access clipboard")?;

        let rendered = template
            .render()
            .context("error while rendering template")?;

        clipboard
            .set_text(rendered)
            .context("unable to access clipboard")?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn create_in_clipboard(&self, template: ProtokollTemplate) -> Result<()> {
        let mut clipboard = Clipboard::new().context("unable to access clipboard")?;

        let rendered = template
            .render()
            .context("error while rendering template")?;

        // stolen from wl-clipboard-rs
        match unsafe { fork() } {
            -1 => panic!("error forking: {:?}", std::io::Error::last_os_error()),
            0 => {
                // Replace STDIN and STDOUT with /dev/null. We won't be using them, and keeping
                // them as is hangs a potential pipeline (i.e. wl-copy hello | cat). Also, simply
                // closing the file descriptors is a bad idea because then they get reused by
                // subsequent temp file opens, which breaks the dup2/close logic during data
                // copying.
                if let Ok(dev_null) = OpenOptions::new().read(true).write(true).open("/dev/null") {
                    let _ = dup2_stdin(&dev_null);
                    let _ = dup2_stdout(&dev_null);
                }

                clipboard
                    .set()
                    .wait()
                    .text(rendered)
                    .expect("unable to write to clipboard");
            }
            _ => (),
        }

        Ok(())
    }

    fn create_in_pad(&self, timestamp: &NaiveDateTime, template: ProtokollTemplate) -> Result<()> {
        let pad_url = timestamp
            .format("https://pad.hhu.de/%Y-%m-%d-FSR-Informatik")
            .to_string();

        println!("opening '{}'", pad_url);

        opener::open_browser(pad_url).context("unable to open pad url")?;

        self.create_in_clipboard(template)
    }

    fn create_from_clipboard(&self, timestamp: &NaiveDateTime) -> Result<()> {
        let mut clipboard = Clipboard::new().context("unable to access clipboard")?;

        let content = clipboard.get_text().context("unable to read clipboard")?;

        self.write_to_file(timestamp, content.as_str())
    }

    fn create_from_pad(
        &self,
        client: &Client,
        pad_url: &Url,
        timestamp: &NaiveDateTime,
    ) -> Result<()> {
        // im not sure how this behaves with non http urls...
        let url_base = pad_url.origin().unicode_serialization();
        let url_path = pad_url.path();

        let endpoint = format!("{}{}/download", url_base, url_path);

        println!("loading pad contents from '{}'", endpoint);

        let response = client
            .get(endpoint)
            .send()
            .context("unable to get pad content")?;

        let content = response
            .text()
            .context("unable to determine content from response")?;

        self.write_to_file(timestamp, content.as_str())
    }
}
