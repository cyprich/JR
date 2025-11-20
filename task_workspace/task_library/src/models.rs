use chrono::{NaiveDate, TimeDelta};
use diesel::{self, Insertable, Queryable, Selectable};

use crate::task::Task;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TaskDb {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub planned_from: String,
    pub planned_duration: i32,
    pub real_from: Option<String>,
    pub real_duration: Option<i32>,
}

impl From<&Task> for TaskDb {
    fn from(value: &Task) -> Self {
        TaskDb {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            priority: value.priority,
            planned_from: value.planned_from.format("%Y%m%d").to_string(),
            planned_duration: value.planned_duration.num_days().try_into().unwrap(),
            real_from: match value.real_from {
                Some(_) => todo!(),
                None => todo!(),
            },
            real_duration: match value.real_duration {
                Some(v) => Some(v.num_days().try_into().unwrap()),
                None => None,
            },
        }
    }
}

impl From<&TaskDb> for Task {
    fn from(value: &TaskDb) -> Self {
        let s = value.planned_from.clone();
        let planned_from = NaiveDate::parse_from_str(s.as_str(), "%Y%m%d").unwrap();
        let s = value.real_from.clone().unwrap();
        let real_from = Some(NaiveDate::parse_from_str(s.as_str(), "%Y%m%d").unwrap());

        Task {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            priority: value.priority,
            planned_from,
            planned_duration: TimeDelta::days(value.planned_duration as i64),
            real_from,

            real_duration: match value.real_duration {
                Some(v) => Some(TimeDelta::days(v as i64)),
                None => todo!(),
            },
        }
    }
}
