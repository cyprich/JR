use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::control::create_task;
use crate::schema::tasks::dsl::tasks;
use crate::schema::tasks::id;
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
pub fn list() -> Vec<Task> {
    let conn = &mut establish_connection();
    let resp = tasks.load::<TaskDb>(conn);

    match resp {
        Ok(val) => val.iter().map(|t| t.into()).collect(),
        Err(val) => {
            println!("Error {val}");
            Vec::new()
        }
    }
}

pub fn list_by_id(task_id: i32) -> Vec<Task> {
    let conn = &mut establish_connection();
    let resp = tasks.filter(id.eq(task_id)).load::<TaskDb>(conn);

    match resp {
        Ok(val) => val.iter().map(|t| t.into()).collect(),
        Err(val) => {
            println!("Error: {}", val);
            Vec::new()
        }
    }
}

pub fn print(show_header: bool) {
    if show_header {
        Task::print_header();
    }

    let resp = list();

    resp.iter().for_each(|t| println!("{}", t));
}

pub fn print_by_id(task_id: i32, show_header: bool) {
    if show_header {
        Task::print_header();
    }

    let resp = list_by_id(task_id);
    resp.iter().for_each(|t| println!("{}", t));
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

pub fn add_task(task: Task) {
    let t: TaskDb = task.into();
    add_task_db(t);
}

pub fn add_task_db(task_db: TaskDb) {
    let conn = &mut establish_connection();
    diesel::insert_into(tasks::table())
        .values(task_db)
        .execute(conn)
        .expect("Error adding to db");
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
