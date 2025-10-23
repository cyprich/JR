use rand::prelude::*;
use std::io::stdin;

fn main() {
    let mut a: i32 = 10;
    a = 11;
    println!("{a}");

    let b = 10;
    let b = 12;
    println!("{b}");

    let b = false;
    println!("{b}");

    let s: String = String::from("ahoj");
    let s: String = "ahoj".to_string();
    let s: String = "ahoj".into();
    println!("{s}");

    let mut s: String = String::new();
    let value = stdin().read_line(&mut s);

    match value {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }

    // let value = match value {
    //     Ok(v) => v,
    //     Err(e) => e, // incompatible types
    // };

    println!("{s}");

    let mut s: String = String::new();
    let value = stdin().read_line(&mut s).expect("Cannot read from console");
    let a: i32 = s.trim().parse().expect("Cannot parse to integer");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let mut rng = rand::rng();
    let a = rng.random_range(1..100);
    println!("{a}");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // random cislo z rozsahu zadaneho od pouzivatela

    let mut min: String = String::new();
    let mut max: String = String::new();
    let mut rng = rand::rng();

    println!("Enter lower value: ");
    let _ = stdin()
        .read_line(&mut min)
        .expect("Couldn't read from console");
    let min: i32 = min.trim().parse().expect("Couldn't convert to int");

    println!("Enter higher value: ");
    let _ = stdin()
        .read_line(&mut max)
        .expect("Couldn't read from console");
    let max: i32 = max.trim().parse().expect("Couldn't convert to int");

    // let n = rng.random_range(min..max);
    println!("You random value is {}", rng.random_range(min..max));
}
