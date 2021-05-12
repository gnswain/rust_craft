use crate::dock::Dock;
use std::sync::{Condvar, Mutex, Arc};

/// foreman.rs
///
/// This file will represent the Foreman. The foreman will pick two different
/// random sets of supplies to be placed on the docks. The foreman will then send a
/// distinct signal for each type of supply delivered. The foreman will not deliver anymore
/// supplies until all the supplies are picked up from the docks.
///
/// # Authors
///
/// * Graham Swain
/// * Mohammed Bukhattala
///
/// # Version
///
/// * 05/12/2021

/// Struct representing the communication between the foreman and the dock, bologna, cheese,
/// and bread messengers.
pub struct Foreman {
    // Shared memory.
    dock: Arc<Mutex<Dock>>,
    // Communication with bologna messenger.
    bologna: Arc<(Mutex<bool>, Condvar)>,
    // Communication with cheese messenger.
    cheese: Arc<(Mutex<bool>, Condvar)>,
    // Communication with bread messenger.
    bread: Arc<(Mutex<bool>, Condvar)>,
}

/// Implementation used to add functionality to the Foreman struct
impl Foreman {
    /// Foreman constructor that takes 4 atomic reference counter arguments to communicate
    /// to the messengers and dock.
    ///
    /// # Arguments
    ///
    /// * `bologna` - Communication link between the foreman and the bologna messenger.
    /// * `cheese` - Communication link between the foreman and the cheese messenger.
    /// * `bread` - Communication link between the foreman and the bread messenger.
    /// * `dock` - Shared memory.
    ///
    /// # Return Value
    ///
    /// * `Foreman` - Foreman Struct
    pub fn new(bologna: Arc<(Mutex<bool>, Condvar)>, cheese: Arc<(Mutex<bool>, Condvar)>,
               bread: Arc<(Mutex<bool>, Condvar)>, dock: Arc<Mutex<Dock>>) -> Foreman {
        Foreman {
            bologna,
            cheese,
            bread,
            dock
        }
    }

    /// This function will randomly place two different types of supplies on the dock and sends
    /// two distinct signals that correspond to the food placed. It will take a random Integer
    /// as an argument that is either 1, 2, or 3.
    ///
    /// # Arguments
    ///
    /// * `self` - Reference to the current object.
    /// * `num` - Random integer (1-3) corresponding to the types of supplies placed on the dock.
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






