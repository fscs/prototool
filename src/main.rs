#![deny(clippy::unwrap_used)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::nursery)]

mod cli;
mod generate;
mod newpost;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = cli::parse();

    match run(args.command) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "[prototool]".red(), e,);

            if e.chain().len() > 1 {
                eprintln!("\t{}", e.root_cause());
            }

            ExitCode::FAILURE
        }
    }
}

fn run(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::GenerateCommand(_) => todo!(),
        cli::Command::NewCommand(subcommand) => {
            let cwd = std::env::current_dir().context("unable to determine working directory")?;
            
            let post_path = newpost::create_post(&cwd, &subcommand.lang, subcommand.path.as_str())?;
            
            println!("Created new post at {}", post_path.to_string_lossy());
            Ok(())
        }
    }
}

fn find_content_dir(root: &Path, lang: &str) -> PathBuf {
    let mut result = root.to_owned();
    result.push("content");
    result.push(lang);

    result
}
