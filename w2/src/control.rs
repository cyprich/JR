pub mod control {
    use std::fs::File;

    use crate::task::TaskManager;

    pub fn list() {
        let tm: TaskManager = serde_json::from_reader(File::open("tasks.json").unwrap()).unwrap();
        tm.list_tasks();
    }
}
