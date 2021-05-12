pub mod foreman;
pub mod dock;
pub mod messenger;
pub mod miner;

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

    let temp = &args[1];
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

    // Docks, shared memory
    let dock = Arc::new(Mutex::new(Dock::new()));   

    // Let's miners signal foreman
    let foreman_arc = Arc::new((Mutex::new(true), Condvar::new()));

    // Communication between foreman and messengers
    let bolognaman_arc = Arc::new((Mutex::new(true), Condvar::new()));
    let cheeseman_arc = Arc::new((Mutex::new(true), Condvar::new()));
    let breadman_arc = Arc::new((Mutex::new(true), Condvar::new()));

    // Communication between messenger and miners
    let bologna_arc = Arc::new((Mutex::new(0), Condvar::new()));
    let cheese_arc = Arc::new((Mutex::new(0), Condvar::new()));
    let bread_arc = Arc::new((Mutex::new(0), Condvar::new()));

// =================== Might want to move these to functions or loops =====================

    // Cloning references so we can move it to threads
    let cloned_bgm = bolognaman_arc.clone();
    let cloned_cm = cheeseman_arc.clone(); 
    let cloned_br = breadman_arc.clone();
    let cloned_dock = dock.clone();
    // ********* Begin Foreman Thread
    thread::spawn(move || {
        let foreman = Foreman::new(cloned_bgm, cloned_cm, cloned_br, cloned_dock);
        let mut rng = rand::thread_rng();
        println!("Foreman Thread ID: {:?}", thread::current().id());
        loop {
            let num = rng.gen_range(1..4);
// TODO sleep
            foreman.place_food(num);

            thread::sleep(Duration::new(1, 0));
        }
    });
    // ********* End Foreman Thread
    
    let foreman_to_bgm = bolognaman_arc.clone();
    let cloned_bgm = bolognaman_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_br = bread_arc.clone();
    // ********* Begin Bolognaman Thread - Notifies miners of bologna
    thread::spawn(move || {
        let bolognaman = Messenger::new(foreman_to_bgm, cloned_c, cloned_br, cloned_bg);
        let (bgm_lock, bgm_cvar) = &*cloned_bgm;

        loop {
            {
                // Should wait while the value in the lock is true
                let mut lock = bgm_cvar.wait_while(bgm_lock.lock().unwrap(), |pending| { *pending }).unwrap();
                println!("===== Bolognaman Thread ID: {:?} =====", thread::current().id());
                bolognaman.supplies_delivered();
                *lock = true;
            }
        }
    });
    // ********* End Bolognaman Thread


    let foreman_to_cm = cheeseman_arc.clone();
    let cloned_cm = cheeseman_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_br = bread_arc.clone();
    // ********* Begin Cheeseman Thread - Notifies miners of cheese
    thread::spawn(move || {
        let bolognaman = Messenger::new(foreman_to_cm, cloned_br, cloned_bg, cloned_c);
        let (cm_lock, cm_cvar) = &*cloned_cm;

        loop {
            {
                // Should wait while the value in the lock is true
                let mut lock = cm_cvar.wait_while(cm_lock.lock().unwrap(), |pending| { *pending }).unwrap();
                println!("===== Cheese Man Thread: {:?} =====", thread::current().id());
                bolognaman.supplies_delivered();
                *lock = true;
            }
        }
    });
    // ********* End Cheeseman Thread


    let foreman_to_brm = breadman_arc.clone();
    let cloned_brm = breadman_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_br = bread_arc.clone();
    // ********* Begin Breadman Thread - Notifies miners of bread
    thread::spawn(move || {
        let bolognaman = Messenger::new(foreman_to_brm, cloned_bg, cloned_c, cloned_br);
        let (cm_lock, cm_cvar) = &*cloned_brm;

        loop {
            {
                // Should wait while the value in the lock is true
                let mut lock = cm_cvar.wait_while(cm_lock.lock().unwrap(), |pending| { *pending }).unwrap();
                println!("===== Bread Man Thread: {:?} =====", thread::current().id());
                bolognaman.supplies_delivered();
                *lock = true;
            }
        }
    });
    // ********* End Breadman Thread


    // ********* Begin Bolognaman Miner Thread
    thread::spawn(move || {
        println!("Thread: {:?}", thread::current().id());

    });
    // ********* End Bolognaman Miner Thread


    // ********* Begin Cheese Miner Thread
    thread::spawn(move || {

        println!("Thread: {:?}", thread::current().id());
    });
    // ********* End Cheese Miner Thread


    // ********* Begin Bread Miner Thread
    thread::spawn(move || {
        println!("Thread: {:?}", thread::current().id());

    });
    // ********* End Bread Miner Thread
    

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
