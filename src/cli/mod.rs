use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod create;
mod generate;

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
    GenerateCommand(generate::GenerateCommand),
    #[command(name = "new")]
    NewCommand(create::NewCommand),
}
