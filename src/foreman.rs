use crate::dock::Dock;
use std::sync::{Condvar, Mutex, Arc};


pub struct Foreman {
    dock: Arc<Mutex<Dock>>,
    bologna: Arc<(Mutex<bool>, Condvar)>,
    cheese: Arc<(Mutex<bool>, Condvar)>,
    bread: Arc<(Mutex<bool>, Condvar)>,
}

impl Foreman {

    pub fn new(bologna: Arc<(Mutex<bool>, Condvar)>, cheese: Arc<(Mutex<bool>, Condvar)>,
               bread: Arc<(Mutex<bool>, Condvar)>, dock: Arc<Mutex<Dock>>) -> Foreman {
        Foreman {
            bologna,
            cheese,
            bread,
            dock
        }
    }

    pub fn place_food(&self, num: u32) {
        println!("Place food");
        match num {
            1 => {
                // This is for bologna
                let (b_lock, b_cvar) = &*self.bread;
                let (c_lock, c_cvar) = &*self.cheese;

                {
                    let temp = &mut *self.dock.lock().unwrap();
                    temp.place_food("Cheese".to_string());
                    temp.place_food("Bread".to_string());
                    
                    *b_lock.lock().unwrap() = true;
                    b_cvar.notify_all();

                    *c_lock.lock().unwrap() = true;
                    c_cvar.notify_all();
                }

 
            }
            2 => {
                // This is for bread
                let (c_lock, c_cvar) = &*self.cheese;
                let (b_lock, b_cvar) = &*self.bologna;
                {
                    let temp = &mut *self.dock.lock().unwrap();
                    temp.place_food("Cheese".to_string());
                    temp.place_food("Bologna".to_string());

                    *b_lock.lock().unwrap() = true;
                    b_cvar.notify_all();
                    
                    *c_lock.lock().unwrap() = true;
                    c_cvar.notify_all();
                }
            }
            3 => {
                // This is for cheese
                let (bg_lock, bg_cvar) = &*self.bologna;
                let (bd_lock, bd_cvar) = &*self.bread;
                {
                    let temp = &mut *self.dock.lock().unwrap();
                    temp.place_food("Bread".to_string());
                    temp.place_food("Bologna".to_string());

                    *bg_lock.lock().unwrap() = true;
                    bg_cvar.notify_all();

                    *bd_lock.lock().unwrap() = true;
                    bd_cvar.notify_all();
                }
            }
            _ => {}
        }
    }
}






