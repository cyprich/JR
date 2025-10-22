use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,

    #[command()]
    pub path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    ListById { id: i32 },
    Add,
    RemoveById { id: i32 },
}
