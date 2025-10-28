use clap::Parser;
use std::path::PathBuf;

use task_library::control;
mod config;
use config::Config;

// mod control;
// use crate::control::{add, interactive, list, list_by_id, remove_by_id, serialize_json};

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

    // pridat command interactive, ktory spusti appku v interktivnom rezime (t.j. bude v cykle
    // alebo pod. citat prikazy od pouzivatela a vykonavat ich kym sa nezada exit alebo tak)

    match args.command {
        config::Commands::List => control::list(&path, args.show_header),
        config::Commands::ListById { id } => control::list_by_id(&path, id, args.show_header),
        config::Commands::Add => control::add(&path),
        config::Commands::RemoveById { id } => control::remove_by_id(&path, id),
        // config::Commands::Interactive => interactive(&path),  // TODO
        config::Commands::Interactive => (),
    };

    // serialize_json(&path, &tm);
}
