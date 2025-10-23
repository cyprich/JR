use clap::Parser;
use std::path::PathBuf;

mod config;
use config::Config;

// mod control;
// use crate::control::{add, interactive, list, list_by_id, remove_by_id, serialize_json};

fn main() {
    let mut args = Config::parse();
    args.path = match args.path {
        Some(val) => Some(val),
        None => {
            let path = match std::env::var("TASKS_PATH") {
                Ok(val) => val,
                Err(_) => String::from("tasks.json"),
            };
            Option::from(PathBuf::from(path))
        }
    };

    // let path = args.path.unwrap();

    // pridat commant interactive, ktory spusti appku v interktivnom rezime (t.j. bude v cykle
    // alebo pod. citat prikazy od pouzivatela a vykonavat ich ky sa nezada exit alebo tak)

    // let tm = match args.command {
    //     config::Commands::List => list(&path, args.show_header),
    //     config::Commands::ListById { id } => list_by_id(&path, id, args.show_header),
    //     config::Commands::Add => add(&path),
    //     config::Commands::RemoveById { id } => remove_by_id(&path, id),
    //     config::Commands::Interactive => interactive(&path),
    // };
    //
    // serialize_json(&path, &tm);
}
