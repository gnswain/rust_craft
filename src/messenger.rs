use std::sync::{Condvar, Mutex, Arc};

pub struct Messenger {
    foreman: Arc<(Mutex<u32>, Condvar)>,
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
            let (lock1, cvar1) = &*self.miner1;
            let (lock2, cvar2) = &*self.miner2;

            let mut m_one = lock1.lock().unwrap();
            let mut m_two = lock2.lock().unwrap();

            *m_one += 1;
            *m_two += 1;

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