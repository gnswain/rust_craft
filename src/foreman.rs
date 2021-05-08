use std::sync::{Condvar, Mutex, Arc};
use std:: thread;
use rand::prelude::*;
use crate::dock::Dock;


pub struct Foreman {
    bologna: Arc<(Mutex<Dock>, Condvar)>,
    cheese: Arc<(Mutex<Dock>, Condvar)>,
    bread: Arc<(Mutex<Dock>, Condvar)>
}

impl Foreman {

    pub fn new(bologna: Arc<(Mutex<Dock>, Condvar)>, cheese: Arc<(Mutex<Dock>, Condvar)>, bread: Arc<(Mutex<Dock>, Condvar)>) -> Foreman {
        Foreman {
            bologna,
            cheese,
            bread
        }
    }

    pub fn place_food(&self) {
        let num = rng.gen_range(1.. 4);

        match num {
            1 => {
                self.cheese.notify_all();
                self.bread.notify_all();
            }
            2 => {
                self.cheese.notify_all();
                self.bologna.notify_all();
            }
            3 => {
                self.bread.notify_all();
                self.bologna.notify_all();
            }
            _ => {}
    }

    // pub fn create(&self) {
    //
    //     thread::spawn(move || {
    //         let mut rng = rand::thread_rng();
    //
    //         loop {
    //             let num = rng.gen_range(1.. 4);
    //
    //             match num {
    //                 1 => {
    //                     self.cheese.notify_all();
    //                     self.bread.notify_all();
    //                 }
    //                 2 => {
    //                     self.cheese.notify_all();
    //                     self.bologna.notify_all();
    //                 }
    //                 3 => {
    //                     self.bread.notify_all();
    //                     self.bologna.notify_all();
    //                 }
    //                 _ => {continue;}
    //             }
    //
    //
    //         }
    //     });

    }
}






