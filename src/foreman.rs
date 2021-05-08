use std::sync::{Condvar, Mutex, Arc};
use std:: thread;
use rand::prelude::*;
use crate::dock::Dock;


pub struct Foreman {
    dock: Arc<Mutex<Dock>>,
    bologna: Arc<(Mutex<bool>, Condvar)>,
    cheese: Arc<(Mutex<bool>, Condvar)>,
    bread: Arc<(Mutex<bool>, Condvar)>,
}

impl Foreman {

    pub fn new(bologna: Arc<(Mutex<bool>, Condvar)>, cheese: Arc<(Mutex<bool>, Condvar)>, bread: Arc<(Mutex<bool>, Condvar)>, dock: Arc<Mutex<Dock>>) -> Foreman {
        Foreman {
            bologna,
            cheese,
            bread,
            dock
        }
    }

    pub fn place_food(&self, num: i32) {
        match num {
            1 => {
                // This is for bologna
                let (b_lock, b_cvar) = &*self.bread;
                let (c_lock, c_cvar) = &*self.cheese;
                {
                    let mut temp = self.dock.lock().unwrap();
                    temp.place_food("Cheese".to_string());
                    temp.place_food("Bread".to_string());
                }
                b_cvar.notify_all();
                c_cvar.notify_all();
            }
            2 => {
                // This is for bread
                let (c_lock, c_cvar) = &*self.cheese;
                let (b_lock, b_cvar) = &*self.bread;
                {
                    let mut temp = self.dock.lock().unwrap();
                    temp.place_food("Cheese".to_string());
                    temp.place_food("Bologna".to_string());
                }
                c_cvar.notify_all();
                b_cvar.notify_all();
            }
            3 => {
                // This is for cheese
                let (bg_lock, bg_cvar) = &*self.bologna;
                let (bd_lock, bd_cvar) = &*self.bread;
                {
                    let mut temp = self.dock.lock().unwrap();
                    temp.place_food("Bread".to_string());
                    temp.place_food("Bologna".to_string());
                }
                bg_cvar.notify_all();
                bd_cvar.notify_all();
            }
            _ => {}
        }
    }
}






