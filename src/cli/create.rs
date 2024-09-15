use anyhow::{Context, Result};
use clap::Args;

use prototool::post;

use super::Runnable;

/// Create a new post.
#[derive(Debug, Args)]
pub struct NewCommand {
    /// Path of the new post. e.g. posts/test.md
    pub path: String,
    /// Under which language the post should be created
    #[arg(short, long, default_value = "de")]
    pub lang: String,
    /// Open the post for editing.  
    #[arg(long, short)]
    pub edit: bool,
    /// Force creation, even if a file already exist
    #[arg(long, short)]
    pub force: bool,
}

impl Runnable for NewCommand {
    fn run(&self) -> Result<()> {
        let cwd = std::env::current_dir().context("unable to determine working directory")?;

        let now = chrono::Local::now().naive_local();
        let content = post::render_post_template(&now).context("error while rendering template")?;

        let post_path = post::create_post(
            content.as_str(),
            &cwd,
            &self.lang,
            self.path.as_str(),
            self.force,
        )?;

        println!("created new post at {}", post_path.to_string_lossy());

        if self.edit {
            post::edit(&post_path)?;
        }

        Ok(())
    }
}
