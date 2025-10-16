use std::{env, path::PathBuf};

use clap::Parser;
// use std::path::Path;

mod task;
use task::TaskManager;

mod config;
use config::Config;

fn main() {
    let mut tm = TaskManager::new();
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

    match args.command {
        config::Commands::List => (),
        config::Commands::ListById { id } => (),
        config::Commands::Add => (),
        config::Commands::RemoveById { id } => (),
    }

    // tm.read_from_csv(Path::new("tasks.csv"), true);
    //
    // println!("\nAll tasks:");
    // tm.print_tasks();
    //
    // println!("\nSorted by planned date:");
    // tm.sort_by_planned_from();
    // tm.print_tasks();
    //
    // println!("\nSorted by planned duration:");
    // tm.sort_by_planned_duration();
    // tm.print_tasks();
    //
    // println!("\nSorted by priority:");
    // tm.sort_by_priority();
    // tm.print_tasks();
    //
    // let serialized_json = serde_json::to_string_pretty(&tm).unwrap();
    // println!("{}\n", serialized_json);
    //
    // let new_task_manager: TaskManager = serde_json::from_str(&serialized_json).unwrap();
    // new_task_manager.print_tasks();
}
