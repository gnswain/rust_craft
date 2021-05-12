pub mod foreman;
pub mod dock;
pub mod messenger;
pub mod miner;

use crate::foreman::Foreman;
use crate::dock::Dock;
use crate::messenger::Messenger;
use crate::miner::Miner;

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
    let cloned_f = foreman_arc.clone();
    let cloned_bgm = bolognaman_arc.clone();
    let cloned_cm = cheeseman_arc.clone(); 
    let cloned_br = breadman_arc.clone();
    let cloned_dock = dock.clone();
    // ********* Begin Foreman Thread
    thread::spawn(move || {
        let foreman = Foreman::new(cloned_bgm, cloned_cm, cloned_br, cloned_dock);
        let (f_lock, f_cvar) = &*cloned_f;
        let mut rng = rand::thread_rng();
        println!("Foreman Thread ID: {:?}", thread::current().id());
        loop {
            // Should wait while the value in the lock is true
            let num = rng.gen_range(1..4);
            println!("Num: {}", num);

            foreman.place_food(num);

            let mut lock = f_cvar.wait_while(f_lock.lock().unwrap(), |pending| { *pending }).unwrap();
            *lock = true;
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
                println!("===== Cheeseman Thread: {:?} =====", thread::current().id());
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
                println!("===== Breadman Thread: {:?} =====", thread::current().id());
                bolognaman.supplies_delivered();
                *lock = true;
            }
        }
    });
    // ********* End Breadman Thread

    let messenger_to_bg = bologna_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_f = foreman_arc.clone();
    let cloned_dock = dock.clone();
    // ********* Begin Bologna Miner Thread
    thread::spawn(move || {
        let mut bologna_miner = Miner::new("Bologna".to_string(), messenger_to_bg, cloned_f, cloned_dock);
        let (bg_lock, bg_cvar) = &*cloned_bg;

        loop {
            let mut lock = bg_cvar.wait_while(bg_lock.lock().unwrap(), |count| {
                *count < 2
            }).unwrap();
            bologna_miner.take_food();
            bologna_miner.signal_foreman();
            println!("##### Bologna Thread: {:?} #####", thread::current().id());
            *lock = 0;
        }
    });
    // ********* End Bologna Miner Thread


    let messenger_to_c = cheese_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_f = foreman_arc.clone();
    let cloned_dock = dock.clone();
    // ********* Begin Cheese Miner Thread
    thread::spawn(move || {
        let mut cheese_miner = Miner::new("Cheese".to_string(), messenger_to_c, cloned_f, cloned_dock);
        let (c_lock, c_cvar) = &*cloned_c;

        loop {
            let mut lock = c_cvar.wait_while(c_lock.lock().unwrap(), |count| {
                *count < 2
            }).unwrap();
            cheese_miner.take_food();
            cheese_miner.signal_foreman();
            println!("##### Cheese Thread: {:?} #####", thread::current().id());
            *lock = 0;
        }
    });
    // ********* End Cheese Miner Thread


    let messenger_to_bd = bread_arc.clone();
    let cloned_bd = bread_arc.clone();
    let cloned_f = foreman_arc.clone();
    let cloned_dock = dock.clone();
    // ********* Begin Bread Miner Thread
    thread::spawn(move || {
        let mut bread_miner = Miner::new("Bread".to_string(), messenger_to_bd, cloned_f, cloned_dock);
        let (bd_lock, bd_cvar) = &*cloned_bd;

        loop {
            let mut lock = bd_cvar.wait_while(bd_lock.lock().unwrap(), |count| {
                *count < 2
            }).unwrap();
            bread_miner.take_food();
            bread_miner.signal_foreman();
            println!("##### Bread Thread: {:?} #####", thread::current().id());
            *lock = 0;
        }
    });
    // ********* End Bread Miner Thread

    let now = Instant::now();
    loop {
        if time > 0 {
            if now.elapsed().as_secs() >= time as u64 {
                exit(0);
            }
        }
    }

}
