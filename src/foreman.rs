use std::sync::{Condvar, Mutex, Arc};
use std:: thread;
use rand::prelude::*;


struct Foreman {
    bologna: Arc<Condvar>,
    cheese: Arc<Condvar>,
    bread: Arc<Condvar>
}

impl Foreman {

    fn new(bologna: Arc<Condvar>, cheese: Arc<Condvar>, bread: Arc<Condvar>) -> Foreman {
        Foreman {
            bologna,
            cheese,
            bread
        }
    }

    fn create() {

        thread::spawn(|| {
            let mut rng = rand::thread_rng();

            loop {
                let num = rng.gen_range(1, 4);

            }
        });

    }


}






