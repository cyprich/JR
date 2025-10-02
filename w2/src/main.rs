use chrono::{Duration, Local, NaiveDate, TimeDelta};

#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
    description: String,
    priority: u32,
    planned_from: NaiveDate,
    planned_duration: TimeDelta,
    real_from: NaiveDate,
    real_duration: TimeDelta,
    // dependency: ,
}

fn main() {
    let t = Task {
        id: 0,
        name: String::from("Create first instance"),
        description: String::from("Create first instance of Task struct"),
        priority: 0,
        planned_from: Local::now().date_naive(),
        planned_duration: Duration::days(1),
        real_from: NaiveDate::from_ymd_opt(2025, 10, 2).unwrap_or_default(),
        real_duration: Duration::days(7),
    };

    println!("{t:#?}");
}
