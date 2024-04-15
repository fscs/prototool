use std::fs;

use anyhow::{Context, Result};
use askama::Template;
use owo_colors::OwoColorize;

use chrono::{DateTime, Utc};
use clap::Args;
use reqwest::blocking::Client;
use url::Url;

use super::Runnable;
use crate::protokoll::ProtokollTemplate;

use crate::post;
use crate::protokoll::{events, raete, tops};

/// Generate a new Protokoll
#[derive(Debug, Args)]
pub struct GenerateCommand {
    /// Endpoint to fetch Tops from
    #[arg(short = 'U', default_value = "https://fscs.hhu.de/")]
    pub endpoint_url: String,
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
}

impl Runnable for GenerateCommand {
    fn run(&self) -> Result<()> {
        let base_url = Url::parse(&self.endpoint_url).context("unable to parse endpoint url")?;
        let client = Client::new();

        println!("[{}] Fetching tops...", "prototool".green(),);
        let tops = tops::fetch_current_tops(&base_url, &client)?;
        let now = chrono::Local::now().to_utc();

        println!("[{}] Fetching räte and withdrawals...", "prototool".green(),);
        let persons = raete::fetch_persons(&base_url, &client, &now)?;
        let abmeldungen = raete::fetch_abmeldungen(&base_url, &client)?;
        let raete = raete::determine_present_räte(&persons, &abmeldungen);

        println!("[{}] Fetching events...", "prototool".green(),);
        let events = events::fetch_calendar_events(&base_url, &client)?;

        let template = ProtokollTemplate {
            datetime: now,
            tops,
            raete,
            events,
        };

        self.create_locally(&now, template)?;

        Ok(())
    }
}

impl GenerateCommand {
    fn create_locally(&self, timestamp: &DateTime<Utc>, template: ProtokollTemplate) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;
        let path = format!("protokolle/{}.md", timestamp.format("%Y-%m-%d"));

        let file_path = post::create_post(&cwd, &self.lang, &path, self.force)?;

        let rendered = template
            .render()
            .context("error while rendering template")?;

        fs::write(&file_path, rendered)?;

        println!(
            "[{}] Created Protokoll at '{}'",
            "prototool".green(),
            file_path.to_string_lossy()
        );

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
}
