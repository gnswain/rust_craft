use crate::dock::Dock;
use std::sync::{Condvar, Mutex, Arc};

/// foreman.rs
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
        println!("\nForeman is placing food.\n");
        match num {
            1 => {
                // This is for bologna
                {
                    let (b_lock, b_cvar) = &*self.bread;
                    let (c_lock, c_cvar) = &*self.cheese;

                    let temp = &mut *self.dock.lock().unwrap();
                    let drop_one = temp.place_food("Bread".to_string());
                    let drop_two = temp.place_food("Cheese".to_string());
                    
                    if drop_one && drop_two {
                        *b_lock.lock().unwrap() = false;
                        b_cvar.notify_all();
    
                        *c_lock.lock().unwrap() = false;
                        c_cvar.notify_all();
                    }
                }

 
            }
            2 => {
                // This is for bread
                {
                    let (c_lock, c_cvar) = &*self.cheese;
                    let (b_lock, b_cvar) = &*self.bologna;

                    let temp = &mut *self.dock.lock().unwrap();
                    let drop_one = temp.place_food("Cheese".to_string());
                    let drop_two = temp.place_food("Bologna".to_string());

                    if drop_one && drop_two {
                        *b_lock.lock().unwrap() = false;
                        b_cvar.notify_all();
                        
                        *c_lock.lock().unwrap() = false;
                        c_cvar.notify_all();
                    }
                }
            }
            3 => {
                // This is for cheese
                {
                    let (bg_lock, bg_cvar) = &*self.bologna;
                    let (bd_lock, bd_cvar) = &*self.bread;
                    
                    let temp = &mut *self.dock.lock().unwrap();
                    let drop_one = temp.place_food("Bread".to_string());
                    let drop_two = temp.place_food("Bologna".to_string());

                    if drop_one && drop_two {
                        *bg_lock.lock().unwrap() = false;
                        bg_cvar.notify_all();
    
                        *bd_lock.lock().unwrap() = false;
                        bd_cvar.notify_all();
                    }
                }
            }
            _ => {}
        }
    }
}






