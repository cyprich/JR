use clap::Parser;
use std::path::PathBuf;

use task_library::{
    control::{self, add, deserialize_json, list, remove_by_id, serialize_json},
    task::{ReadTask, TaskManager},
};
mod config;
use config::Config;

mod console_reader;
use console_reader::ConsoleReader;

use crate::console_reader::read_string;

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
    let mut task_manager = deserialize_json(&path);

    match args.command {
        config::Commands::List => control::list(&task_manager, args.show_header),
        config::Commands::ListById { id } => {
            control::list_by_id(&task_manager, id, args.show_header)
        }
        config::Commands::Add => control::add(&mut task_manager, &ConsoleReader),
        config::Commands::RemoveById { id } => control::remove_by_id(&mut task_manager, id),
        config::Commands::Interactive => {
            interactive(&mut task_manager, args.show_header, &ConsoleReader)
        }
    };

    serialize_json(&path, &task_manager);
}

fn interactive(tm: &mut TaskManager, show_header: bool, reader: &impl ReadTask) {
    println!("Launching in interactive mode... Press ctrl+c to quit\n");

    loop {
        let string = read_string("Command: ");
        let string = string.trim();

        match string {
            "quit" => break,
            "list" => list(tm, show_header),
            "add" => add(tm, reader),
            // "remove" => remove_by_id(tm, id),
            _ => println!("Unknown command... available commands are: list, add, quit"),
        }
    }
}
