use std::sync::{Condvar, Mutex, Arc};

/// This file will represent our messengers. The messengers sends signals to the
/// miners to notify them that there's food on the dock.
///
/// # Authors
///
/// * Graham Swain
/// * Mohammed Bukhattala
///
/// # Version
///
/// * 05/12/2021

/// Messenger struct to represent communication between the messenger and the miners.
pub struct Messenger {
    // This are how the messengers communicate with each miner
    miner1: Arc<(Mutex<u32>, Condvar)>,
    miner2: Arc<(Mutex<u32>, Condvar)>,
    miner3: Arc<(Mutex<u32>, Condvar)>
}

/// Implementation used to add functionality to the messenger.
impl Messenger {
    /// Messenger constructor that takes 3 atomic reference counter arguments to communicate
    /// to the different miners.
    ///
    /// # Arguments
    ///
    /// * `miner1` - Communication link between the messenger and the miner
    /// * `miner2` - Communication link between the messenger and the miner
    /// * `miner3` - Communication link between the messenger and the miner
    ///
    /// # Return Value
    ///
    /// * `Messenger` - a Messenger struct.
    pub fn new(miner1: Arc<(Mutex<u32>, Condvar)>, miner2: Arc<(Mutex<u32>, Condvar)>, 
               miner3: Arc<(Mutex<u32>, Condvar)>) -> Messenger {
        Messenger {
            miner1,
            miner2,
            miner3
        }
    }

    /// This function notifies the miners that food has been delivered to the dock.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the current object.
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

                *m_two = 0;
                
                *(*self.miner3).0.lock().unwrap() = 0;
            } else if *m_two == 2 {
                cvar2.notify_all();
                *m_one = 0;
                
                *(*self.miner3).0.lock().unwrap() = 0;
            }
        }
    }
}