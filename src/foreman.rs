use std::sync::{Condvar, Mutex, Arc};
use std:: thread;
use rand::prelude::*;
use crate::dock::Dock;


pub struct Foreman {
    dock: Arc<Mutex<Dock>>,
    bologna: Arc<Condvar>,
    cheese: Arc<Condvar>,
    bread: Arc<Condvar>,
}

impl Foreman {

    pub fn new(bologna: Arc<Condvar>, cheese: Arc<Condvar>, bread: Arc<Condvar>, dock: Arc<Mutex<Dock>>) -> Foreman {
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
                {
                    let mut temp = self.dock.lock().unwrap();
                    temp.place_food("Cheese".to_string());
                    temp.place_food("Bread".to_string());
                }
                self.bread.notify_all();
                self.cheese.notify_all();
            }
            2 => {
                // This is for bread
                {
                    let mut temp = self.dock.lock().unwrap();
                    temp.place_food("Cheese".to_string());
                    temp.place_food("Bologna".to_string());
                }
                self.cheese.notify_all();
                self.bologna.notify_all();
            }
            3 => {
                // This is for cheese
                {
                    let mut temp = self.dock.lock().unwrap();
                    temp.place_food("Bread".to_string());
                    temp.place_food("Bologna".to_string());
                }
                self.bologna.notify_all();
                self.bread.notify_all();
            }
            _ => {}
        }
    }
}






