pub mod control {
    use std::{fs::File, path::PathBuf};

    use crate::task::task::TaskManager;

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
        let result = serde_json::to_writer(
            File::create(path).unwrap(), &tm);
        match result {
            Ok(_) => (),
            Err(e) => println!("{e}")
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
}
