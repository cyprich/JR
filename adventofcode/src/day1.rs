use std::{fs, path::PathBuf};

pub fn main() {
    println!("Day 1");

    let mut pointing_at = 50;
    let mut result = 0;

    let filename = "input1.txt";
    let lines = fs::read_to_string(PathBuf::from(filename)).expect("Couln't read file");

    for line in lines.split("\n") {
        let command = line.chars().next().unwrap_or(' ');

        let number: String = line.chars().skip(1).collect();
        let number: i32 = number.trim().parse().unwrap_or(0);

        match command {
            'R' => pointing_at += number,
            'L' => pointing_at -= number,
            _ => println!("Couldn't parse line: {}", line),
        }

        while pointing_at >= 100 {
            pointing_at -= 100;
        }

        while pointing_at < 0 {
            pointing_at += 100;
        }

        if pointing_at == 0 {
            result += 1;
        }
    }

    println!("Pointing at: {}", pointing_at);
    println!("Result: {}", result);
}
