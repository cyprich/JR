use std::{fs::File, path::PathBuf};

use crate::task::{Task, TaskManager, create_task_from_console};

pub fn deserialize_json(path: &PathBuf) -> TaskManager {
    let tm: TaskManager;

    match std::fs::exists(path) {
        Ok(true) => tm = serde_json::from_reader(File::open(path).unwrap()).unwrap(),
        Ok(false) => {
            println!(
                "Path \"{}\" not found. Creating new TaskManager",
                path.to_str().unwrap()
            );
            tm = TaskManager::new();
        }
        Err(e) => {
            panic!("{}", e)
        }
    }

    tm
}

pub fn serialize_json(path: &PathBuf, tm: &TaskManager) {
    let result = serde_json::to_writer(File::create(path).unwrap(), &tm);
    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

pub fn list(path: &PathBuf, show_header: bool) {
    let tm = deserialize_json(path);
    tm.list_tasks(show_header);
    serialize_json(path, &tm);
}

pub fn list_by_id(path: &PathBuf, id: i32, show_header: bool) {
    let tm = deserialize_json(path);
    let task = tm.list_task_by_id(id);

    match task {
        Some(val) => {
            if show_header {
                Task::print_header();
            }
            val.print();
        }
        None => println!("Task with ID {id} not found"),
    }

    serialize_json(path, &tm);
}

// pub fn add(path: &PathBuf, reader: impl ReadFromUser) {
pub fn add(path: &PathBuf) {
    let mut tm = deserialize_json(path);
    let task = create_task_from_console();
    tm.add_task(task);
}

pub fn remove_by_id(path: &PathBuf, id: i32) {
    let mut tm = deserialize_json(path);
    tm.remove_task_by_id(id);
    serialize_json(path, &tm);
}

// TODO
// pub fn interactive(path: &PathBuf, show_header: bool) {
//     let mut tm = deserialize_json(path);
//
//     println!("Launching in interactive mode...");
//
//     list(path, show_header);
//
//     loop {
//         add(path);
//         println!();
//         list(path, show_header);
//         let choice = read_string("Do you want to add another task? [y/N]: ");
//         let choice = choice.to_lowercase();
//
//         if choice != "y" || choice != "yes" {
//             break;
//         }
//     }
// }
