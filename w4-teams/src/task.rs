pub mod control;

pub mod task {
    use std::{fs::File, io::{BufRead, BufReader}, path::Path, task};

    use chrono::{NaiveDate, TimeDelta};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Task {
        pub id: i32,
        pub nazov: String,
        pub popis: String,
        pub priorita: i32,
        pub planovany_zaciatok: NaiveDate,
        pub skutocny_zaciatok: Option<NaiveDate>,
        pub planovane_trvanie: TimeDelta,
        pub skutocne_trvanie: Option<TimeDelta>,
    }

    //1. upravme strukturu Task tak, aby skutocny zaciatok a skutocne trvanie nebolo nutne zadat pri vytvarani tasku
    //2. vytvorte metodu pre typ Task, ktora vypocita a vrati koncovy datum (na zaklade planovaneho zaciatku a trvania)
    impl Task {
        pub fn vypocitaj_planovany_koniec(&self) -> NaiveDate {
            self.planovany_zaciatok + self.planovane_trvanie
        }
        pub fn vypocitaj_skutocny_koniec(&self) -> Option<NaiveDate> {
            match self.skutocny_zaciatok {
                Some(zaciatok) => match self.skutocne_trvanie {
                    Some(trvanie) => Option::from(zaciatok + trvanie),
                    None => None,
                },
                None => None,
            }
        }

        pub fn print_task(&self) {
            let skutocny_zaciatok = match self.skutocny_zaciatok {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            };
            let skutocny_koniec = match self.vypocitaj_skutocny_koniec() {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            };
            println!(
                "{}: {}\t{}\t{}\t{}\t{}",
                self.id,
                self.nazov,
                self.planovany_zaciatok,
                self.vypocitaj_planovany_koniec(),
                skutocny_zaciatok,
                skutocny_koniec
            );
        } //spravte metodu pre TaskManager, ktora vypise vsetky tasky (vyuzite funkciu print_task)
    }

    #[derive(Serialize, Deserialize, Debug)]
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

        //metoda, ktora pozicia task na zaklade id tasku
        pub fn get_task_by_id(&self, id: i32) -> Option<&Task> {
            let result = self.tasks.iter().find(|t| t.id == id);
            result
        }

        pub fn print_all_tasks(&self) {
            for task in &self.tasks {
                task.print_task();
            }
        }

        // 1. metoda, ktora zoradi tasky podla planovanej dlzky
        pub fn sort_tasks_by_planned_duration(&mut self) {
            self.tasks
                .sort_by(|t1, t2| t1.planovane_trvanie.cmp(&t2.planovane_trvanie));
        }
        // 2. metoda, ktora zoradi tasky podla planovaneho zaciatku
        pub fn sort_tasks_by_planned_start(&mut self) {
            self.tasks
                .sort_by(|t1, t2| t1.planovany_zaciatok.cmp(&t2.planovany_zaciatok));
        }

        pub fn read_from_txt_file(&mut self, file_path: &Path) {
            let file = File::open(file_path).expect("Cannot open file!");
            let file_lines = BufReader::new(file).lines();
            for line in file_lines {
                match line {
                    Ok(line) => {
                        let task = Self::_process_line(&line);
                        self.add_task(task);
                        println!("{line}")
                    },
                    Err(e) => println!("{e}")
                }
            }
        }

        fn _process_line(line: &String) -> Task {
            let split: Vec<&str> = line.split(',').collect();
            let task = Task {
                id: split[0].parse().unwrap(),
                nazov: String::from(split[1]),
                popis: String::from(split[2]),
                priorita: split[3].parse().unwrap(),
                planovany_zaciatok: NaiveDate::parse_from_str(
                    split[4], "%d.%m.%Y").expect("Cannot parse from string"),
                skutocny_zaciatok: Option::from(NaiveDate::parse_from_str(
                    split[5], "%d.%m.%Y").expect("Cannot parse from string")),
                planovane_trvanie: TimeDelta::days(split[6].parse().unwrap()),
                skutocne_trvanie: Option::from(TimeDelta::days(split[7].parse().unwrap()))
            };
            task
        }
    }
}
