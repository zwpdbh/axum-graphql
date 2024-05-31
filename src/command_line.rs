use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author = "zhaowei", version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    StartServer {
        #[arg(long, short)]
        port: String,
    },
    Sqlx,
    Ex02 {
        #[clap(subcommand)]
        case: ExCase,
    },
    Ex03 {
        case: ValueEnumCase,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ExCase {
    Case01 {
        #[arg(short, long)]
        name: String,
    },
    Case02,
}
#[derive(Debug, Clone, ValueEnum)]
pub enum ValueEnumCase {
    Case01,
    Case02,
    Case03,
}
