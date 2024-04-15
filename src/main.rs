#![deny(clippy::unwrap_used)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::nursery)]

mod cli;
mod post;
mod protokoll;

use anyhow::Result;
use owo_colors::OwoColorize;

use std::process::ExitCode;

use cli::Runnable;

fn main() -> ExitCode {
    let args = cli::parse();

    match run(args.command) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}\n {}", "error:".red(), e,);

            if e.chain().len() > 1 {
                eprintln!(" {}", e.root_cause());
            }

            ExitCode::FAILURE
        }
    }
}

fn run(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::GenerateCommand(x) => x.run(),
        cli::Command::NewCommand(x) => x.run(),
    }
}
