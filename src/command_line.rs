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
    Sqlx {
        #[clap(subcommand)]
        case: SqlCase,
    },
    Ex03 {
        case: MigrationFolder,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum SqlCase {
    Test,
    Migrate {
        #[arg(long, short)]
        folder: MigrationFolder,
    },
    Bookstore {
        #[clap(subcommand)]
        example: BookstoreEx,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum MigrationFolder {
    Bookstore,
}

#[derive(Debug, Clone, Subcommand)]
pub enum BookstoreEx {
    Create,
    Update,
    Read {
        #[arg(short)]
        v: ExVersion,
    },
    Transaction,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ExVersion {
    V1,
    V2,
    V3,
    V4,
}
