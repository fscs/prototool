use std::fs::OpenOptions;

use anyhow::{anyhow, Context, Result};
use arboard::Clipboard;
use askama::Template;
use chrono::{DateTime, FixedOffset, Local, NaiveTime};
use clap::{ArgGroup, Args};
use inquire::MultiSelect;
use reqwest::blocking::Client;
use url::Url;

#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
#[cfg(target_os = "linux")]
use libc::fork;
#[cfg(target_os = "linux")]
use rustix::stdio::{dup2_stdin, dup2_stdout};

use super::Runnable;
use prototool::{
    post,
    protokoll::{self, events, person, sitzung},
    Event, PersonWithAbmeldung, ProtokollTemplate, SitzungKind,
};

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
    #[arg(long, alias = "tc")]
    pub to_clipboard: bool,
    /// Load the protokoll content from the system clipboard
    #[arg(long, alias = "fc")]
    pub from_clipboard: bool,
    /// Copies the protokolls content into the system clipboard and opens an appropriate
    /// pad url in the webbrowser
    #[arg(long, alias = "tp")]
    pub to_pad: bool,
    /// Load the protokoll content from a hedgedoc note
    #[arg(long, value_name = "PAD_URL", alias = "fp")]
    pub from_pad: Option<Url>,
    /// Dont Ask for Presence
    #[arg(long)]
    pub no_ask_presence: bool,
}

impl Runnable for GenerateCommand {
    fn run(&self) -> Result<()> {
        let client = Client::new();

        if self.from_clipboard {
            return self.create_from_clipboard();
        } else if let Some(pad_url) = &self.from_pad {
            return self.create_from_pad(&client, pad_url);
        }

        #[allow(clippy::unwrap_used)]
        let now = chrono::Local::now()
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .unwrap()
            .fixed_offset();

        let template = self.build_template(&client, now)?;

        // create_in_clipboard might fork, so we drop this here
        drop(client);

        if self.to_clipboard {
            self.create_in_clipboard(template)
        } else if self.to_pad {
            self.create_in_pad(template)
        } else {
            self.create_locally(template)
        }
    }
}

impl GenerateCommand {
    fn build_template(
        &self,
        client: &Client,
        sitzung_date: DateTime<FixedOffset>,
    ) -> Result<ProtokollTemplate> {
        println!("fetching sitzung...");
        let sitzung = sitzung::fetch_sitzung(&self.endpoint_url, client, sitzung_date)?;

        println!("fetching räte and withdrawals...");
        let raete = person::fetch_raete(&self.endpoint_url, client)?;
        let abmeldungen = person::fetch_abmeldungen(&self.endpoint_url, client, &sitzung)?;
        let mut raete_and_abmeldung = person::determine_abgemeldet_räte(&raete, &abmeldungen);

        if !self.no_ask_presence {
            self.ask_present_räte(&mut raete_and_abmeldung)?;
        }

        println!("fetching events...");
        let events = events::fetch_calendar_events(&self.endpoint_url, client)?
            .into_iter()
            .map(|e| Event {
                title: e.title,
                location: e.location,
                start: e.start.with_timezone::<Local>(&Local).into(),
            })
            .collect();

        return Ok(ProtokollTemplate {
            sitzung,
            raete: raete_and_abmeldung,
            events,
        });
    }

    fn ask_present_räte(&self, räte: &mut [PersonWithAbmeldung]) -> Result<()> {
        let selected = MultiSelect::new("select present räte:", räte.to_vec()).prompt()?;

        for rat in räte {
            rat.anwesend = selected.iter().any(|s| s.id == rat.id);
        }

        Ok(())
    }

    fn write_to_file(&self, content: &str) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;

        let markdown_opts = markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::Constructs::gfm()
            },
            ..markdown::ParseOptions::default()
        };

        let mdast = markdown::to_mdast(content, &markdown_opts)
            .map_err(|_| anyhow!("unable to parse pad contents"))?;

        let frontmatter = protokoll::find_frontmatter(&mdast).unwrap();

        let timestamp = protokoll::find_protokoll_date(&frontmatter)
            .context("unable to determine protokoll date")?;

        let sitzung_kind = frontmatter.sitzung_kind.unwrap_or(SitzungKind::Normal);

        let prefix = match sitzung_kind {
            SitzungKind::Normal | SitzungKind::Ersatz | SitzungKind::Dringlichkeit => "",
            SitzungKind::VV | SitzungKind::WahlVV => "vv-",
            sitzung::SitzungKind::Konsti => "konsti-",
        };

        let path = format!(
            "protokolle/{}/{}-{}protokoll.md",
            timestamp.format("%Y"),
            timestamp.format("%m-%d"),
            prefix,
        );

        let file_path = post::create_post(content, &cwd, &self.lang, &path, self.force)?;

        println!("created protokoll at '{}'", file_path.to_string_lossy());

        if self.edit {
            post::edit(&file_path)?
        }

        Ok(())
    }

    fn create_locally(&self, template: ProtokollTemplate) -> Result<()> {
        let rendered = template
            .render()
            .context("error while rendering template")?;

        self.write_to_file(rendered.as_str())
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

    fn create_in_pad(&self, template: ProtokollTemplate) -> Result<()> {
        let pad_url = template
            .sitzung
            .datetime
            .format("https://pad.hhu.de/%Y-%m-%d-FSR-Informatik")
            .to_string();

        println!("opening '{}'", pad_url);

        opener::open_browser(pad_url).context("unable to open pad url")?;

        self.create_in_clipboard(template)
    }

    fn create_from_clipboard(&self) -> Result<()> {
        let mut clipboard = Clipboard::new().context("unable to access clipboard")?;

        let content = clipboard.get_text().context("unable to read clipboard")?;

        self.write_to_file(content.as_str())
    }

    fn create_from_pad(&self, client: &Client, pad_url: &Url) -> Result<()> {
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

        self.write_to_file(content.as_str())
    }
}
