use std::{fs, num::ParseIntError, path::PathBuf};

pub fn main() {
    println!("Day 2");

    let filename = "example2.txt";
    let s = fs::read_to_string(PathBuf::from(filename)).expect("Couldn't read from file");

    let mut invalids: Vec<String> = Vec::new();

    for raw_range in s.split(",") {
        let mut splitted = raw_range.split("-");
        let smaller = parse_number(splitted.next().unwrap()).unwrap();
        let larger = parse_number(splitted.next().unwrap()).unwrap();

        let range = smaller..larger;

        for i in range {
            let i = i.to_string();

            if detect_repeating(i.clone()) {
                invalids.push(i.clone());
            }
        }
    }

    let result = invalids.join("");
    println!("{}", result);
}

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn detect_repeating(s: String) -> bool {
    for i in 1..s.len() / 2 {
        let split = s.split_at(i).0;
        println!("{}", split);
    }

    false
}
