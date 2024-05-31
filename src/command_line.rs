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
        #[arg(long, short)]
        example: BookStoreEx,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum MigrationFolder {
    Bookstore,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BookStoreEx {
    Create,
    Update,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ValueEnumCase {
    Case01,
    Case02,
    Case03,
}
