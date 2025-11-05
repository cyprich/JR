pub mod control;

pub mod task {
    // use std::ffi::os_str::Display;

    use chrono::{NaiveDate, TimeDelta};
    use serde::{Deserialize, Serialize};

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

    pub trait ReadTask {
        fn read_id(&self, message: &str) -> i32;
        fn read_name(&self, message: &str) -> String;
        fn read_description(&self, message: &str) -> String;
        fn read_priority(&self, message: &str) -> i32;
        fn read_planned_from(&self, message: &str) -> NaiveDate;
        fn read_planned_duration(&self, message: &str) -> TimeDelta;
        fn read_real_from(&self, message: &str) -> Option<NaiveDate>;
        fn read_real_duration(&self, message: &str) -> Option<TimeDelta>;
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

        pub fn print(&self) {
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

        pub fn print_header() {
            println!(
                "Task ID, Priority, Planned from, Planned to, Real from, Real to, Description
----------------------------------------------------------------------------"
            );
        }
    }

    impl std::fmt::Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "#{} {} ({}) with priority {}, planned start: {} for {} days, real start: {} for {} days",
                self.id,
                self.name,
                self.description,
                self.priority,
                self.planned_from,
                self.planned_duration.num_days(),
                match self.real_from {
                    Some(val) => val.to_string(),
                    None => String::from("-"),
                },
                match self.real_duration {
                    Some(val) => val.num_days().to_string(),
                    None => String::from("-"),
                    // None => 0,
                },
            )
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

        pub fn get_tasks(&self) -> &Vec<Task> {
            &self.tasks
        }

        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        pub fn remove_task_by_id(&mut self, id: i32) {
            for index in 0..self.tasks.len() {
                let current_task = self.tasks.get(index);
                match current_task {
                    Some(t) => {
                        if t.id == id {
                            self.tasks.remove(index);
                        }
                    }
                    None => (),
                }
            }
        }

        pub fn list_task_by_id(&self, id: i32) -> Option<&Task> {
            self.tasks.iter().find(|t| t.id == id)
        }

        pub fn list_tasks(&self, show_header: bool) {
            if show_header {
                Task::print_header();
            }

            for i in self.tasks.iter() {
                i.print();
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
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
