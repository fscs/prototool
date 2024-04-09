use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use owo_colors::OwoColorize;
use reqwest::blocking::Client;
use url::Url;

use crate::post;
use crate::protokoll::{self, events, raete, tops};

pub trait Runnable {
    fn run(&self) -> Result<()>;
}

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "gen")]
    GenerateCommand(GenerateCommand),
    #[command(name = "new")]
    NewCommand(NewCommand),
}

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
}

impl Runnable for GenerateCommand {
    fn run(&self) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;

        let base_url = Url::parse(&self.endpoint_url).context("unable to parse endpoint url")?;
        let client = Client::new();

        println!("[{}] Fetching tops...", "prototool".green(),);
        let tops = tops::fetch_current_tops(&base_url, &client)?;
        let now = chrono::Local::now().naive_local();

        println!("[{}] Fetching räte and withdrawals...", "prototool".green(),);
        let persons = raete::fetch_persons(&base_url, &client, &now)?;
        let abmeldungen = raete::fetch_abmeldungen(&base_url, &client)?;
        let räte = raete::determine_present_räte(&persons, &abmeldungen);

        println!("[{}] Fetching events...", "prototool".green(),);
        let events = events::fetch_calendar_events(&base_url, &client)?;

        let path = format!("protokolle/{}.md", now.format("%Y-%m-%d"));

        let file_path = post::create_post(&cwd, &self.lang, &path)?;

        println!(
            "[{}] Created Protokoll at '{}'",
            "prototool".green(),
            file_path.to_string_lossy()
        );

        protokoll::write_protokoll_template(&file_path, tops, räte, events, &now)?;

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

/// Create a new post.
#[derive(Debug, Args)]
pub struct NewCommand {
    /// Path of the new post. e.g. posts/test.md
    pub path: String,
    /// Under which language the post should be created
    #[arg(short, long, default_value = "de")]
    pub lang: String,
    /// Open the post for editing.  
    /// Optionally takes the editor to use, falls back to $EDITOR otherwise
    #[arg(long, short)]
    pub edit: Option<Option<String>>,
}

impl Runnable for NewCommand {
    fn run(&self) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;

        let post_path = post::create_post(&cwd, &self.lang, self.path.as_str())?;

        println!(
            "[{}] Created new post at {}",
            "prototool".green(),
            post_path.to_string_lossy()
        );

        let now = chrono::Local::now().naive_local();

        post::write_post_template(&post_path, now)?;

        if let Some(maybe_editor) = &self.edit {
            let editor = match maybe_editor {
                Some(x) => x.to_owned(),
                None => std::env::var("EDITOR")
                    .context("unable to determine editor. wasnt specified and $EDITOR isnt set")?,
            };

            post::edit(&post_path, &editor)?
        }

        Ok(())
    }
}
