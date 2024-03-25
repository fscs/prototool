use clap::{Args, Parser, Subcommand};

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

#[derive(Debug, Args)]
pub struct NewCommand {
    pub path: String,
    #[arg(short, long, default_value = "de")]
    pub lang: String,
    #[arg(long, short)]
    pub no_edit: bool
}
