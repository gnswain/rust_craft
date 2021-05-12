use crate::dock::Dock;
use std::sync::{Condvar, Mutex, Arc};
use std::thread;
use std::time::{Duration};
use rand::prelude::*;

/// miner.rs
///
/// This file will represent the different miners. Miners will wait until all the supplies
/// they need are available on the dock. Once they have all the food they need, they will
/// signal the foreman that the dock is clear. Miners do not communicate with each other.
///
/// # Authors
///
/// * Graham Swain
/// * Mohammed Bukhattala
///
/// # Version
///
/// 05/12/2021
pub struct Miner {
    // Name of miners
    name: String,
    // Communication between the miner and the foreman
    foreman: Arc<(Mutex<bool>, Condvar)>,
    // Shared memory
    dock: Arc<Mutex<Dock>>
}

impl Miner {
    /// Miner constructor that takes in a String to specify the different miners. It also takes
    /// 2 atomic reference counter arguments to communicate with the foreman and share memory.
    ///
    /// # Arguments
    ///
    /// * `name` - String representing the type of miner
    /// * `foreman` - Communication link between the miner and the foreman
    /// * `dock` - Shared memory
    ///
    /// # Return Value
    ///
    /// * `Miner` - Miner Struct
    pub fn new(name: String, foreman: Arc<(Mutex<bool>, Condvar)>, dock: Arc<Mutex<Dock>>) -> Miner {
        Miner {
            name,
            foreman,
            dock
        }
    }

    /// This function will lock the dock so that only one miner can access food on the dock. It
    /// will then clear the dock. the lock will be dropped once we exit the scope.
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the current object.
    pub fn take_food(&mut self) {
        println!("\n  ~~~ {} is taking food. ~~~", self.name);
        {
            let dock_access = &mut *self.dock.lock().unwrap();
            let dock_str = (&*dock_access.to_string()).to_string();
            println!("\n  ~~~ {} picks up {}. ~~~", self.name, dock_str);
            dock_access.take_food();
        }
    }

    /// This function is called when the miners pick up food from the dock. The miners
    /// will make food for a random amount of time(0-4).
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the current object
    pub fn make_food(&self) {
        let mut rng = rand::thread_rng();
        let milli = rng.gen_range(0..4000);
        println!("\n    ^^^ {} is MAKING food, it will take {} milliseconds. ^^^", self.name, milli);
        thread::sleep(Duration::from_millis(milli));
    }

    /// This function is called after the make_food() function. The miners will eat the food
    /// for a random amount of time (0-4 sec).
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the current object.
    pub fn eat_food(&self) {
        let mut rng = rand::thread_rng();
        let milli = rng.gen_range(0..4000);
        println!("\n    *** {} is EATING food, it will take {} milliseconds. ***", self.name, milli);
        thread::sleep(Duration::from_millis(milli));
    }

    pub fn signal_foreman(&mut self) {
        println!("\n      <<< {} is signalling foreman. >>>", self.name);
        {
            *(*self.foreman).0.lock().unwrap() = false;
        }
        (*self.foreman).1.notify_all();
    }
}