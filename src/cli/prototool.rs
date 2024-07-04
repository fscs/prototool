#![deny(clippy::unwrap_used)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::nursery)]

use anyhow::Result;
use owo_colors::OwoColorize;
use clap::Parser;
use clap::Subcommand;

mod create;
mod generate;

pub trait Runnable {
    fn run(&self) -> Result<()>;
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
    GenerateCommand(generate::GenerateCommand),
    #[command(name = "new")]
    NewCommand(create::NewCommand),
}

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Cli::parse();

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

fn run(command: Command) -> Result<()> {
    match command {
        Command::GenerateCommand(x) => x.run(),
        Command::NewCommand(x) => x.run(),
    }
}
