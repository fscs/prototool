use clap::{Args, Parser, Subcommand};

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "gen")]
    GenerateCommand(GenerateCommand),
}

#[derive(Debug, Args)]
pub struct GenerateCommand {
    #[arg(short = 'U', default_value = "https://fscs.hhu.de/api")]
    endpoint_url: String,
}
