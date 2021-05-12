use std::sync::{Condvar, Mutex, Arc};

pub struct Messenger {
    // Probably will take this one out and handle it in main. It's how the foreman wakes the messenger up
    foreman: Arc<(Mutex<u32>, Condvar)>,

    // This are how the messengers communicate with each miner
    miner1: Arc<(Mutex<u32>, Condvar)>,
    miner2: Arc<(Mutex<u32>, Condvar)>,
    miner3: Arc<(Mutex<u32>, Condvar)>
}

impl Messenger {
    pub fn new(foreman: Arc<(Mutex<u32>, Condvar)>, miner1: Arc<(Mutex<u32>, Condvar)>, miner2: Arc<(Mutex<u32>, Condvar)>, miner3: Arc<(Mutex<u32>, Condvar)>) -> Messenger {
        Messenger {
            foreman,
            miner1,
            miner2,
            miner3
        }
    }

    pub fn supplies_delivered(&self) {
        {
            // When a signal is sent, the miner will wake up and notify the correct miners
            // ex. If this is the Cheeseman, he'll wake up when cheese is deilvered to the dock
            // and increment the counter for Bread and Bologna miners
            let (lock1, cvar1) = &*self.miner1;
            let (lock2, cvar2) = &*self.miner2;

            let mut m_one = lock1.lock().unwrap();
            let mut m_two = lock2.lock().unwrap();

            *m_one += 1;
            *m_two += 1;

            // If either of the counters hits 2, notify the correct miner and set all counters to 0
            if *m_one == 2 {
                cvar1.notify_all();

                *m_one = 0;
                *m_two = 0;
                
                *(*self.miner3).0.lock().unwrap() = 0;
            } else if *m_two == 2 {
                cvar2.notify_all();

                *m_one = 0;
                *m_two = 0;
                
                *(*self.miner3).0.lock().unwrap() = 0;
            }
        }
    }
}