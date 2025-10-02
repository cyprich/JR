use std::io::{Write, stdin, stdout};

use chrono::{Duration, Local, NaiveDate, TimeDelta};

#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    description: String,
    priority: i32,
    planned_from: NaiveDate,
    planned_duration: TimeDelta,
    real_from: Option<NaiveDate>,
    real_duration: Option<TimeDelta>,
    // dependency: ,
}

impl Task {
    fn calculate_planned_end(&self) -> NaiveDate {
        self.planned_from + self.planned_duration
    }

    fn calculate_real_end(&self) -> Option<NaiveDate> {
        match self.real_from {
            Some(from) => match self.real_duration {
                Some(dur) => Option::from(from + dur),
                None => None,
            },
            None => None,
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

    buf.trim().to_string()
}

fn read_number(message: &str) -> i32 {
    let result: i32 = read_string(message)
        .trim()
        .parse()
        .expect("Couldn't convert to i32");
    result
}

fn read_date(message: &str) -> NaiveDate {
    NaiveDate::parse_from_str(read_string(message).as_str(), "%d.%m.%Y")
        .expect("Couldn't parse date")
}

fn read_timedelta(message: &str) -> TimeDelta {
    TimeDelta::days(read_number(message).into())
}

fn create_task_from_console() -> Task {
    let id = read_number("  ID of Task: ");
    let name = read_string("  Name of Task: ");
    let description = read_string("  Description of Task: ");
    let priority = read_number("  Priority of Task: ");
    let planned_from = read_date("  Date from: ");
    let planned_duration = read_timedelta("  Planned duration: ");
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

fn main() {
    let t = create_task_from_console();
    let planned_end = t.calculate_planned_end();
    let real_end = t.calculate_real_end();

    println!("{t:#?}\n{planned_end:?}\n{real_end:?}");
}
