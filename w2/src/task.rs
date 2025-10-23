use std::{
    fs::File,
    io::{BufRead, BufReader, Write, stdin, stdout},
    path::Path,
};

use chrono::{NaiveDate, TimeDelta};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub planned_from: NaiveDate,
    pub planned_duration: TimeDelta,
    pub real_from: Option<NaiveDate>,
    pub real_duration: Option<TimeDelta>,
    // dependency:
}

impl Task {
    pub fn calculate_planned_end(&self) -> NaiveDate {
        self.planned_from + self.planned_duration
    }

    pub fn calculate_real_end(&self) -> Option<NaiveDate> {
        match self.real_from {
            Some(from) => match self.real_duration {
                Some(dur) => Option::from(from + dur),
                None => None,
            },
            None => None,
        }
    }

    pub fn print(&self, show_header: bool) {
        let real_from = match self.real_from {
            Some(val) => val.to_string(),
            None => String::from("-"),
        };

        let real_to = match self.calculate_real_end() {
            Some(val) => val.to_string(),
            None => String::from("-"),
        };

        let real_duration = match self.real_duration {
            Some(val) => val.num_days().to_string(),
            None => String::from("-"),
        };

        let header = "Task ID, Priority, Planned from, Planned to, Real from, Real to, Description";
        if show_header {
            println!("{header}")
        };

        println!(
            "{}, {}, {}, {}, {}, {}, {}, {} {}, {}",
            self.id,
            self.name,
            self.priority,
            self.planned_from,
            self.calculate_planned_end(),
            self.planned_duration.num_days(),
            real_from,
            real_to,
            real_duration,
            self.description
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: vec![] }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task_by_id(&mut self, id: i32) {
        self.tasks.remove(id as usize);
    }

    pub fn list_task_by_id(&self, id: i32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn list_tasks(&self, show_header: bool) {
        let mut is_first = true;

        for i in self.tasks.iter() {
            i.print(is_first && show_header);
            is_first = false;
        }
    }

    pub fn sort_by_planned_duration(&mut self) {
        self.tasks
            .sort_by(|a, b| a.planned_duration.cmp(&b.planned_duration));
    }

    pub fn sort_by_planned_from(&mut self) {
        self.tasks
            .sort_by(|a, b| a.planned_from.cmp(&b.planned_from));
    }

    pub fn sort_by_priority(&mut self) {
        self.tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    pub fn read_from_csv(&mut self, file_path: &Path, has_header: bool) {
        let file = File::open(file_path).expect("Cannot open file {file_path}");
        let file_lines = BufReader::new(file).lines();

        let lines = file_lines.skip(if has_header { 1 } else { 0 });

        for line in lines {
            match line {
                Ok(val) => {
                    let t = self.parse_line(&val);
                    self.add_task(t);
                }
                Err(e) => {
                    println!("{e}")
                }
            }
        }
    }

    fn parse_line(&self, line: &str) -> Task {
        let split: Vec<&str> = line.split(',').collect();

        let real_from = if split[6].is_empty() {
            None
        } else {
            Some(parse_date(split[6]))
        };

        let real_duration = if split[7].is_empty() {
            None
        } else {
            Some(parse_timedelta(split[7]))
        };

        Task {
            id: parse_number(split[0]),
            name: parse_string(split[1]),
            description: parse_string(split[2]),
            priority: parse_number(split[3]),
            planned_from: parse_date(split[4]),
            planned_duration: parse_timedelta(split[5]),
            real_from,
            real_duration,
        }
    }
}

fn read_string(message: &str) -> String {
    print!("{message}");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Cannot read from console");

    parse_string(&buf)
}

fn parse_string(string: &str) -> String {
    string.trim().to_string()
}

fn read_number(message: &str) -> i32 {
    let string = read_string(message);
    parse_number(&string)
}

fn parse_number(string: &str) -> i32 {
    string.trim().parse().expect("Couldn't convert to i32")
}

fn read_date(message: &str) -> NaiveDate {
    let string = read_string(message);
    parse_date(&string)
}

fn parse_date(string: &str) -> NaiveDate {
    NaiveDate::parse_from_str(string, "%d.%m.%Y").expect("Couldn't parse date")
}

fn read_timedelta(message: &str) -> TimeDelta {
    let string = read_string(message);
    parse_timedelta(&string)
}

fn parse_timedelta(string: &str) -> TimeDelta {
    let number = parse_number(string);
    TimeDelta::days(number.into())
}

pub fn create_task_from_console() -> Task {
    let id = read_number("ID of Task: ");
    let name = read_string("Name of Task: ");
    let description = read_string("Description of Task: ");
    let priority = read_number("Priority of Task: ");
    let planned_from = read_date("Date from (ex. 1.1.2025): ");
    let planned_duration = read_timedelta("Planned duration (days): ");
    let real_from = None;
    let real_duration = None;

    Task {
        id,
        name,
        description,
        priority,
        planned_from,
        planned_duration,
        real_from,
        real_duration,
    }
}
