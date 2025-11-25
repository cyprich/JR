use std::path::PathBuf;

use ratatui::widgets::ListState;
use task_library::{control::deserialize_json, task::TaskManager};

#[derive(Debug)]
pub struct TaskList {
    pub task_manager: TaskManager,
    pub state: ListState,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            task_manager: TaskManager::new(),
            state: ListState::default(),
        }
    }

    pub fn open(mut self, path: &PathBuf) -> Self {
        self.task_manager = deserialize_json(path);
        self
    } 
}
