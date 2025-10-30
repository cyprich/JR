use std::{fs::File, path::PathBuf};

use crate::task::{ReadTask, Task, TaskManager};

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
pub fn add(path: &PathBuf, reader: &impl ReadTask) {
    let mut tm = deserialize_json(path);
    let task = Task {
        id: reader.read_id("ID of Task: "),
        name: reader.read_name("Name of Task: "),
        description: reader.read_description("Description of Task: "),
        priority: reader.read_priority("Priority of Task: "),
        planned_from: reader.read_planned_from("Planned date from (ex. 1.1.2025): "),
        planned_duration: reader.read_planned_duration("Planned duration (whole days): "),
        real_from: reader.read_real_from("Real date from (ex. 1.1.2025) (optional): "),
        real_duration: reader.read_real_duration("Real duration (whole days) (optional): "),
    };

    tm.add_task(task);
    serialize_json(path, &tm);
}

pub fn remove_by_id(path: &PathBuf, id: i32) {
    let mut tm = deserialize_json(path);
    tm.remove_task_by_id(id);
    serialize_json(path, &tm);
}

// TODO
pub fn interactive(path: &PathBuf, show_header: bool, reader: &impl ReadTask) {
    println!("Launching in interactive mode... Press ctrl+c to quit");

    list(path, show_header);

    loop {
        println!();
        add(path, reader);
        println!();
        list(path, show_header);
    }
}
