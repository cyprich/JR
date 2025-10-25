use std::{fs::File, path::PathBuf};

use crate::task::{ReadTaskFromUser, Task, TaskManager};

fn deserialize_json(path: &PathBuf) -> TaskManager {
    let tm: TaskManager;
    match std::fs::exists(path) {
        Ok(true) => {
            tm = serde_json::from_reader(File::open(path).unwrap()).unwrap();
        }
        Ok(false) => {
            println!(
                "Subor s taskami na zadanej ceste neexistuje. Zadana cesta bola {}",
                path.to_str().unwrap()
            );
            println!("Vytvaram novy task manager.");
            tm = TaskManager::new();
        }
        Err(e) => panic!("{e}"),
    }
    tm
}

fn serialize_json(path: &PathBuf, tm: &TaskManager) {
    let result = serde_json::to_writer(File::create(path).unwrap(), &tm);
    match result {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}

pub fn list_tasks(path: &PathBuf) {
    let tm = deserialize_json(path);
    tm.print_all_tasks();
}

pub fn show_task_by_id(path: &PathBuf, task_id: i32) {
    let tm = deserialize_json(path);
    // podla id najdem task
    let task = tm.get_task_by_id(task_id);
    // vypisem ho
    match task {
        Some(task) => task.print_task(),
        None => println!("Task so zadanym ID nebol najdeny."),
    }
}

pub fn add_task(path: &PathBuf, reader: impl ReadTaskFromUser) {
    let mut tm = deserialize_json(path);
    let task = Task {
        id: reader.read_id("Zadaj ID tasku: "),
        nazov: reader.read_nazov("Zadaj nazov tasku: "),
        popis: reader.read_popis("Zadaj popis tasku: "),
        priorita: reader.read_priorita("Zadaj prioritu: "),
        planovany_zaciatok: reader.read_planovany_zaciatok("Zadaj planovany zaciatok: "),
        skutocny_zaciatok: reader.read_skutocny_zaciatok("Zadaj skutocny zaciatok: "),
        planovane_trvanie: reader.read_planovane_trvanie("Zadaj planovane trvanie: "),
        skutocne_trvanie: reader.read_skutocne_trvanie("Zadaj skutocne trvanie: "),
    };
    tm.add_task(task);
    serialize_json(path, &tm);
}

pub fn remove_task_by_id(path: &PathBuf, task_id: i32) {
    let mut tm = deserialize_json(path);
    let result = tm.remove_by_id(task_id);
    match result {
        Some(_) => println!("Task s ID {} odstraneny", task_id),
        None => println!("Task s ID {} nebol najdeny", task_id),
    };
    serialize_json(path, &tm);
}
