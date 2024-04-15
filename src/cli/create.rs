use anyhow::{Context, Result};
use clap::Args;
use owo_colors::OwoColorize;

use super::Runnable;
use crate::post;

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
    /// Force creation, even if a file already exist
    #[arg(long, short)]
    pub force: bool,
}

impl Runnable for NewCommand {
    fn run(&self) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;

        let post_path = post::create_post(&cwd, &self.lang, self.path.as_str(), self.force)?;

        println!(
            "[{}] Created new post at {}",
            "prototool".green(),
            post_path.to_string_lossy()
        );

        let now = chrono::Local::now().naive_local();

        post::write_post_template(&post_path, &now)?;

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
