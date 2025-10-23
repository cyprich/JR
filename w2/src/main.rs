use clap::Parser;
use std::path::PathBuf;

mod task;

mod config;
use config::Config;

mod control;
use crate::control::{add, list, list_by_id, remove_by_id, serialize_json};

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

    let path = args.path.unwrap();

    // pri moznosti list, ak pouzivatel zada prepinac --show-header, zobrazi sa okrem zoznamu
    // taskov aj hlavicka - popis jednotlivych stlpcov, ktore sa vypisu
    //
    // pri pridavani taskov upravit nacitavanie taym sposobom, ze ked pouzivatel zada `-`  pri
    // nepoivnnych atributoch, tak tieto sa nastavia na hodnotu none

    let tm = match args.command {
        config::Commands::List => list(&path, args.show_header),
        config::Commands::ListById { id } => list_by_id(&path, id, args.show_header),
        config::Commands::Add => add(&path),
        config::Commands::RemoveById { id } => remove_by_id(&path, id),
    };

    serialize_json(&path, &tm);
}
