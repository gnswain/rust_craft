use crate::dock::Dock;
use std::sync::{Condvar, Mutex, Arc};
use std::thread;
use std::time::{Duration};
use rand::prelude::*;

pub struct Miner {
    /// Name of miners
    name: String,

    // Might also go in main
    foreman: Arc<(Mutex<bool>, Condvar)>,

    dock: Arc<Mutex<Dock>>
}

impl Miner {
    pub fn new(name: String, foreman: Arc<(Mutex<bool>, Condvar)>, dock: Arc<Mutex<Dock>>) -> Miner {
        Miner {
            name,
            foreman,
            dock
        }
    }

    pub fn take_food(&mut self) {
        {
            let dock_access = &mut *self.dock.lock().unwrap();
            let dock_str = (&*dock_access.to_string()).to_string();
            println!("\n{} picks up {}.\n", self.name, dock_str);
            dock_access.take_food();
        }
    }

    pub fn make_food(&self) {
        println!("\n{} is making food.\n", self.name);
        let mut rng = rand::thread_rng();
        let milli = rng.gen_range(0..4000);
        println!("{} is sleeping for {} milliseconds.", self.name, milli);
        thread::sleep(Duration::from_millis(milli));
    }

    pub fn signal_foreman(&mut self) {
        println!("\n{} signalling foreman.\n", self.name);
        {
            *(*self.foreman).0.lock().unwrap() = false;
        }
        (*self.foreman).1.notify_all();
    }
}