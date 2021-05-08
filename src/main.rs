pub mod foreman;
pub mod dock;
use crate::foreman::Foreman;
use crate::dock::Dock;

use std::env::args;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = args().collect();
    let foreman = Forman::new();

    // let mut dick = Dock::new();
    // dick.place_food("Cheese".to_string());


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

    // let test = thread::spawn(|| {
    //     let mut i = 0;
    //     loop {
    //         println!("printing shit: {}", i);
    //         i = i + 1;
    //     }
    // });

    //thread::park_timeout(Duration::from_secs_f32(time as f32));


    let now = Instant::now();
    loop {
        if now.elapsed().as_secs() >= time as u64 {
            exit(0);
        }
    }

}
