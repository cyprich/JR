use std::path::PathBuf;

use ratatui::widgets::ListState;
use task_library::{
    control::deserialize_json,
    task::{Task, TaskManager},
};

#[derive(Debug)]
pub struct TaskList {
    pub task_manager: TaskManager,
    pub state: ListState,
    pub rendered_task: Option<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            task_manager: TaskManager::new(),
            state: ListState::default(),
            rendered_task: None,
        }
    }

    pub fn open(mut self, path: &PathBuf) -> Self {
        self.task_manager = deserialize_json(path);
        self
    }

    pub fn update_selected_task(&mut self) {
        self.rendered_task = match self.state.selected() {
            Some(val) => self.task_manager.get_task_by_index(val).cloned(),
            None => None,
        }
    }
}
