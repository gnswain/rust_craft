pub mod foreman;
pub mod dock;
pub mod messenger;
use crate::foreman::Foreman;
use crate::dock::Dock;
use crate::messenger::Messenger;

use std::sync::{Condvar, Mutex, Arc};
use std::env::args;
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};
use rand::prelude::*;

fn main() {
    let args: Vec<String> = args().collect();

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

    let dock = Arc::new(Mutex::new(Dock::new()));
    let bologna_man_arc = Arc::new((Mutex::new(0), Condvar::new()));
    let cheeseman_arc = Arc::new((Mutex::new(0), Condvar::new()));
    let breadman_arc = Arc::new((Mutex::new(0), Condvar::new()));

    let foreman = Foreman::new(bologna_man_arc, cheeseman_arc, breadman_arc, dock);

    // ********* Begin Foreman Thread
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        loop {
            let num = rng.gen_range(1..4);

            foreman.place_food(num);

            thread::sleep(Duration::new(1, 0));
        }


    });
    // ********* End Foreman Thread

    //thread::park_timeout(Duration::from_secs_f32(time as f32));


    let now = Instant::now();
    loop {
        if time > 0 {
            if now.elapsed().as_secs() >= time as u64 {
                exit(0);
            }
        }
    }

}
