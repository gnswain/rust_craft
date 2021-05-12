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
use std::time::{Instant};
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
    
    // Setting up and creating Foreman thread
    let cloned_f = foreman_arc.clone();
    let cloned_bgm = bolognaman_arc.clone();
    let cloned_cm = cheeseman_arc.clone(); 
    let cloned_br = breadman_arc.clone();
    let cloned_dock = dock.clone();
    spawn_foreman(cloned_f, cloned_bgm, cloned_cm, cloned_br, cloned_dock);
    
    // Setting up and creating Bolognaman thread
    let cloned_bgm = bolognaman_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_br = bread_arc.clone();
    spawn_messenger("Bolognaman".to_string(), cloned_bgm, cloned_c, cloned_br, cloned_bg);

    // Setting up and creating Cheeseman thread
    let cloned_cm = cheeseman_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_br = bread_arc.clone();
    spawn_messenger("Cheeseman".to_string(), cloned_cm, cloned_br, cloned_bg, cloned_c);

    // Setting up and creating Breadman thread
    let cloned_brm = breadman_arc.clone();
    let cloned_bg = bologna_arc.clone();
    let cloned_c = cheese_arc.clone();
    let cloned_br = bread_arc.clone();
    spawn_messenger("Breadman".to_string(), cloned_brm, cloned_bg, cloned_c, cloned_br);

    // Setting up and creating Bologna thread
    let cloned_bg = bologna_arc.clone();
    let cloned_f = foreman_arc.clone();
    let cloned_dock = dock.clone();
    spawn_miner("Bologna".to_string(), cloned_bg, cloned_f, cloned_dock);

    // Setting up and creating Cheese thread
    let cloned_c = cheese_arc.clone();
    let cloned_f = foreman_arc.clone();
    let cloned_dock = dock.clone();
    spawn_miner("Cheese".to_string(), cloned_c, cloned_f, cloned_dock);

    // Setting up and creating Bread thread
    let cloned_bd = bread_arc.clone();
    let cloned_f = foreman_arc.clone();
    let cloned_dock = dock.clone();
    spawn_miner("Bread".to_string(), cloned_bd, cloned_f, cloned_dock);

    // Main thread keeping track of time
    let now = Instant::now();
    loop {
        if time > 0 {
            if now.elapsed().as_secs() >= time as u64 {
                exit(0);
            }
        }
    }
}

/// This function is used to spawn the foreman thread. It takes in 4 atomic references to
/// communicate with the different messengers. It also takes in an atomic reference to shared
/// memory. The foreman thread will place food on the dock and sleeps until awaken again by
/// the miners' signal.
///
/// # Arguments
///
/// * `foreman_arc` - Communication link for the foreman.
/// * `bolognaman_arc` - Atomic reference to communicate with the bologna messenger.
/// * `cheeseman_arc` - Atomic reference to communicate with the cheese messenger.
/// * `breadman_arc` - Atomic reference to communicate with the bread messenger.
/// * `dock` - Shared memory.
fn spawn_foreman(foreman_arc: Arc<(Mutex<bool>, Condvar)>, bolognaman_arc: Arc<(Mutex<bool>, Condvar)>,
                 cheeseman_arc: Arc<(Mutex<bool>, Condvar)>, breadman_arc: Arc<(Mutex<bool>, Condvar)>,
                 dock: Arc<Mutex<Dock>>) {
        // ********* Begin Foreman Thread
        thread::spawn(move || {
            let foreman = Foreman::new(bolognaman_arc, cheeseman_arc, breadman_arc, dock);
            let (f_lock, f_cvar) = &*foreman_arc;
            let mut rng = rand::thread_rng();
            
            loop {
                // Should wait while the value in the lock is true
                println!("\n~~~ Foreman is waking up ~~~");
                let num = rng.gen_range(1..4);
    
                println!("\n------------------------------------");
                foreman.place_food(num);
                println!("\n------------------------------------\n");
    
                let mut lock = f_cvar.wait_while(f_lock.lock().unwrap(), |pending| { *pending }).unwrap();
                *lock = true;
            }
        });
        // ********* End Foreman Thread
}

/// This function is used to spawn the messenger(s) thread.
fn spawn_messenger(name: String, messenger: Arc<(Mutex<bool>, Condvar)>, miner1: Arc<(Mutex<u32>, Condvar)>,
                   miner2: Arc<(Mutex<u32>, Condvar)>, miner3: Arc<(Mutex<u32>, Condvar)>) {
    // ********* Begin Messenger Thread
    thread::spawn(move || {
        let dealer = Messenger::new(miner1, miner2, miner3);
        let (lock, cvar) = &*messenger;

        loop {
            {
                // Should wait while the value in the lock is true
                let mut lock = cvar.wait_while(lock.lock().unwrap(), |pending| { *pending }).unwrap();
                println!("\n=== {} wakes up. ===", name);
                dealer.supplies_delivered();
                *lock = true;
            }
        }
    });
    // ********* End Messenger Thread
}

/// this function is used to spawn the miner(s).
fn spawn_miner(name: String, miner_arc: Arc<(Mutex<u32>, Condvar)>, foreman: Arc<(Mutex<bool>, Condvar)>,
               dock: Arc<Mutex<Dock>>) {
    // ********* Begin Miner Thread
    thread::spawn(move || {
        let temp = name.clone();
        let mut miner = Miner::new(temp, foreman, dock);
        let (lock, cvar) = &*miner_arc;

        loop {
            println!("\n--- {} wakes up. ---", name);
            {
                let mut lock = cvar.wait_while(lock.lock().unwrap(), |count| {
                    *count < 2
                }).unwrap();
                *lock = 0;
            }
            miner.take_food();
            miner.signal_foreman();
            miner.make_food();
            miner.eat_food();
            println!("\n{} needs food.", name);
        }
    });
    // ********* End Miner Thread
}
