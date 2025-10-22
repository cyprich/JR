use std::{fs::File, path::PathBuf};

use crate::task::{TaskManager, create_task_from_console};

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

pub fn list(path: &PathBuf) -> TaskManager {
    let tm = deserialize_json(path);
    tm.list_tasks();
    tm
}

pub fn list_by_id(path: &PathBuf, id: i32) -> TaskManager {
    let tm = deserialize_json(path);
    let task = tm.list_task_by_id(id);

    match task {
        Some(val) => val.print(),
        None => println!("Task with ID {id} not found"),
    }

    tm
}

pub fn add(path: &PathBuf) -> TaskManager {
    let mut tm = deserialize_json(path);
    let task = create_task_from_console();
    tm.add_task(task);
    tm
}

pub fn remove_by_id(path: &PathBuf, id: i32) -> TaskManager {
    let mut tm = deserialize_json(path);
    tm.remove_task_by_id(id);
    tm
}
