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
use crate::protokoll::ProtokollTemplate;

use crate::post;
use crate::protokoll::{events, raete, tops};
use crate::sitzung;

/// Generate a new Protokoll
#[derive(Debug, Args)]
#[clap(group(
            ArgGroup::new("import_export")
                .args(&["to_clipboard", "from_clipboard"]),
        ))]
pub struct GenerateCommand {
    /// Endpoint to fetch Tops from
    #[arg(short = 'U', default_value = "https://fscs.hhu.de/")]
    pub endpoint_url: Url,
    /// Under which language the protokoll should be created
    #[arg(short, long, default_value = "de")]
    pub lang: String,
    /// Open the protokoll for editing.  
    /// Optionally takes the editor to use, falls back to $EDITOR otherwise
    #[arg(long, short)]
    pub edit: Option<Option<String>>,
    /// Force creation, even if a file already exist
    #[arg(long, short)]
    pub force: bool,
    /// Generate the protokoll into the system clipboard
    #[arg(long)]
    pub to_clipboard: bool,
    /// Load the protokoll content from the system clipboard
    #[arg(long)]
    pub from_clipboard: bool,
}

impl Runnable for GenerateCommand {
    fn run(&self) -> Result<()> {
        let client = Client::new();

        let next_sitzung = sitzung::fetch_next_sitzung(&self.endpoint_url, &client)?;
        let timestamp = next_sitzung.date;

        println!("Fetching tops...");
        let tops = tops::fetch_current_tops(&self.endpoint_url, &client)?;

        println!("Fetching räte and withdrawals...");
        let persons = raete::fetch_persons(&self.endpoint_url, &client, &timestamp)?;
        let abmeldungen = raete::fetch_abmeldungen(&self.endpoint_url, &client)?;
        let raete = raete::determine_present_räte(&persons, &abmeldungen);

        println!("Fetching events...");
        let events = events::fetch_calendar_events(&self.endpoint_url, &client)?;

        let template = ProtokollTemplate {
            datetime: timestamp,
            tops,
            raete,
            events,
        };

        // if running in to_clipboard mode, this might lead to a panic (because we're forking)
        // so we do it now
        drop(client);

        if self.to_clipboard {
            self.create_in_clipboard(template)?;
        } else if self.from_clipboard {
            unimplemented!();
        } else {
            self.create_locally(&timestamp, template)?;
        }

        Ok(())
    }
}

impl GenerateCommand {
    fn create_locally(&self, timestamp: &NaiveDateTime, template: ProtokollTemplate) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;
        let path = format!(
            "protokolle/{}/{}-protokoll.md",
            timestamp.format("%Y"),
            timestamp.format("%m-%d"),
        );

        let file_path = post::create_post(&cwd, &self.lang, &path, self.force)?;

        let rendered = template
            .render()
            .context("error while rendering template")?;

        fs::write(&file_path, rendered)?;

        println!("Created Protokoll at '{}'", file_path.to_string_lossy());

        if let Some(maybe_editor) = &self.edit {
            let editor = match maybe_editor {
                Some(x) => x.to_owned(),
                None => std::env::var("EDITOR")
                    .context("unable to determine editor. wasnt specified and $EDITOR isnt set")?,
            };

            post::edit(&file_path, &editor)?
        }

        Ok(())
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
}
