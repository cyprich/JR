use diesel::associations::HasTable;
use diesel::deserialize::Result;
use diesel::prelude::*;

use crate::control::create_task;
use crate::schema::tasks::dsl::tasks;
use crate::schema::tasks::id;
use crate::schema::tasks::name;
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
pub fn list(show_header: bool) {
    let conn = &mut establish_connection();
    let task = tasks.load::<TaskDb>(conn);

    if show_header {
        Task::print_header();
    }

    match task {
        Ok(val) => val.iter().for_each(|t| {
            let t: Task = t.into();
            println!("{}", t)
        }),
        Err(val) => println!("Error: {}", val),
    }
}

pub fn list_by_id(task_id: i32, show_header: bool) {
    let conn = &mut establish_connection();
    let task = tasks.filter(id.eq(task_id)).load::<TaskDb>(conn);

    if show_header {
        Task::print_header();
    }

    match task {
        Ok(val) => val.iter().for_each(|t| {
            let t: Task = t.into();
            println!("{}", t);
        }),
        Err(val) => println!("Error: {}", val),
    }
}

pub fn add(reader: &impl ReadTask) {
    let conn = &mut establish_connection();
    let task: Task = create_task(reader);
    let task_db: TaskDb = task.into();
    diesel::insert_into(tasks::table())
        .values(task_db)
        .execute(conn)
        .expect("Error adding tasks to db");
}

pub fn remove_by_id(task_id: i32) {
    let conn = &mut establish_connection();
    let result = diesel::delete(tasks.filter(id.eq(task_id))).execute(conn);
    // let result = diesel::delete(diesel::QueryDsl::filter(tasks, id.eq(task_id))).execute(conn);

    match result {
        Ok(val) => println!("Succesfully removed {val} rows"),
        Err(val) => println!("Error: {val}"),
    }
}
