use std::sync::{Condvar, Mutex, Arc};
use std:: thread;
use rand::prelude::*;


pub struct Foreman {
    bologna: Arc<(Mutex<String>, Condvar)>,
    cheese: Arc<(Mutex<String>, Condvar)>,
    bread: Arc<(Mutex<String>, Condvar)>
}

impl Foreman {

    pub fn new(bologna: Arc<(Mutex<String>, Condvar)>, cheese: Arc<(Mutex<String>, Condvar)>, bread: Arc<(Mutex<String>, Condvar)>) -> Foreman {
        Foreman {
            bologna,
            cheese,
            bread
        }
    }

    pub fn create(&self) {

        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
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
                    _ => {continue;}
                }


            }
        });

    }
}






