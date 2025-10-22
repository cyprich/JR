use std::{
    fs::read,
    io::{Read, Write, stdin, stdout},
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::{Local, NaiveDate, TimeDelta};
use clap::Parser;
use task::{
    control::control,
    task::{Task, TaskManager},
};
use task_manager_config::task_manager_config::{AppConfig, Commands};

mod task;
mod task_manager_config;

fn main() {
    // spustim aplikaciu s prikazom print-tasks - aplikacia nacita tasky z json suboru a vypise ich
    // spustim aplikaciu s prikazom add-task - aplikacia nacita udaje pre novy task od pouzivatela
    // a prida ho do suboru s taskami
    // spustim aplikaciu s prikazom get-task a argumentom id tasku - aplikacia vypise prislusny task
    let mut args = AppConfig::parse();

    // 1. budeme hladat cestu ako argument programu
    args.path = match args.path {
        Some(path) => Some(path),
        None => {
            // 2. ak to nie je medzi argumentami programu, zoberieme to z premennej prostredia
            let path = match std::env::var("TASK_PATH") {
                Ok(p) => p,
                // 3. ak to nie je ani tam, tak pouzijeme nejaku defaultnu cestu (napr. tasky.json)
                Err(_) => String::from("tasky.json"),
            };
            Option::from(PathBuf::from(path))
        }
    };

    match args.command {
        Commands::ListTasks => control::list_tasks(&args.path.unwrap()),
        Commands::ShowTaskById { task_id } => {
            control::show_task_by_id(&args.path.unwrap(), task_id)
        }
        Commands::AddTask => (),
        Commands::RemoveTaskById { task_id } => (),
    };

    // let mut tm = TaskManager::new();
    // tm.read_from_txt_file(Path::new("./tasky.txt"));
    // tm.print_all_tasks();

    // let task = tm.get_task_by_id(5);
    // println!("{task:?}");

    // let serialized_json = serde_json::to_string_pretty(&tm).unwrap();
    // println!("{serialized_json}");
    // let new_task_manager: TaskManager = serde_json::from_str(&serialized_json).unwrap();
    // println!("{new_task_manager:?}");
}

fn read_string(message: &str) -> String {
    print!("{message}");
    stdout().flush().expect("Error while flushing");
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer
}
// nacitanie cisla
fn read_i32(message: &str) -> i32 {
    let mut buffer = read_string(message);
    let cislo = buffer.trim().parse().expect("Cannot be parsed to i32");
    cislo
}
// nacitanie datumu
fn read_date(message: &str) -> NaiveDate {
    let mut buffer = read_string(message);
    let datum =
        NaiveDate::parse_from_str(buffer.trim(), "%d.%m.%Y").expect("Cannot parse from string");
    datum
}
