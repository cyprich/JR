use std::{env, path::PathBuf};

use clap::Parser;
use std::path::Path;

mod task;
use task::TaskManager;

mod config;
mod control;

use config::Config;

use crate::control::{add, list, list_by_id, remove_by_id, serialize_json};

fn main() {
    let mut args = Config::parse();
    args.path = match args.path {
        Some(val) => Some(val),
        None => {
            let path = match std::env::var("TASKS_PATH") {
                Ok(val) => val,
                Err(_) => String::from("tasks.json"),
            };
            Option::from(PathBuf::from(path))
        }
    };

    let path = args.path.unwrap();

    let tm = match args.command {
        config::Commands::List => list(&path),
        config::Commands::ListById { id } => list_by_id(&path, id),
        config::Commands::Add => add(&path),
        config::Commands::RemoveById { id } => remove_by_id(&path, id),
    };

    serialize_json(&path, &tm);
}
