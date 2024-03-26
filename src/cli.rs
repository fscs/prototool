use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use owo_colors::OwoColorize;

use crate::post;

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

#[derive(Debug, Args)]
pub struct GenerateCommand {
    #[arg(short = 'U', default_value = "https://fscs.hhu.de/api")]
    pub endpoint_url: String,
}

impl Runnable for GenerateCommand {
    fn run(&self) -> Result<()> {
        todo!()
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
