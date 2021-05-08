pub mod foreman;

use std::env::args;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = args().collect();
    //let man: foreman = Forman::new();

    if args.len() != 3 {
        println!("\nUsage: cargo run [amount of time] [T or F]");
        exit(0);
    }

    let mut temp = &args[1];
    let time: i32 = match temp.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("\nTime must be an integer.");
            exit(0);
        }
    };

    let mut write_to_file = false;

    if args[2].eq_ignore_ascii_case("T") {
        write_to_file = true;
        if time < 1 {
            println!("\nTime must be positive if writing to a file.");
            exit(0);
        }
    } else if !args[2].eq_ignore_ascii_case("F") {
        println!("\nSecond argument must be either 'T' or 'F' (True/False)");
        exit(0);
    }

    let now = Instant::now();
    loop {
        if now.elapsed().as_secs() >= time as u64 {
            exit(0);
        }
    }












}
