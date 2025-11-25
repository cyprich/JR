use crate::schema::tasks::dsl::tasks;
use crate::task::ReadTaskFromUser;
use crate::{db::estabilish_connection, models::TaskDb, task::TaskManager};
use diesel::prelude::*;
use diesel::associations::HasTable;

use super::managed;

pub fn create_from_db() -> TaskManager {
    let connection = &mut estabilish_connection();
    let result: Vec<TaskDb> = tasks
        .limit(10)
        .load(connection)
        .expect("Error loading tasks from db");
    let converted = result.iter().map(|t| t.into());
    let mut tm = TaskManager::new();
    for t in converted {
        tm.add_task(t);
    }
    tm
}

pub fn add_task(reader: &impl ReadTaskFromUser) {
    let connection = &mut estabilish_connection();
    let task = managed::create_task(reader);
    let task_db: TaskDb = task.into();
    diesel::insert_into(tasks::table())
        .values(task_db)
        .execute(connection)
        .expect("Error adding task to db!");
}
