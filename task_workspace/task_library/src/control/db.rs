use diesel::RunQueryDsl;
use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::schema::tasks::dsl::tasks;
use crate::task::ReadTask;
use crate::task::Task;
use crate::{db::establish_connection, models::TaskDb, task::TaskManager};

pub fn create_from_db() -> TaskManager {
    let mut conn = establish_connection();
    let result: Vec<TaskDb> = tasks
        .limit(10)
        .load(&mut conn)
        .expect("Error loading tasks from db");

    let converted = result.iter().map(|t| t.into());

    let mut tm = TaskManager::new();

    converted.for_each(|t| tm.add_task(t));

    tm
}

pub fn add_task(reader: &impl ReadTask) {
    let conn = &mut establish_connection();
    let task: Task = create_task(reader);
    let task_db: TaskDb = task.into();
    diesel::insert_into(tasks::table())
        .values(task_db)
        .execute(conn)
        .expect("Error adding tasks to db")
}
