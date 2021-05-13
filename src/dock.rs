use std::sync::{Mutex, Arc};
use std::fs::File;
use std::io::Write;

/// Dock.rs
///
/// This file will represent our dock where the food coming from the foreman will be placed.
///
/// # Authors
///
/// * Graham Swain
/// * Mohammed Bukhattala
///
/// # Version
///
/// * 05/12/2021

/// Struct representing the food currently placed on the dock.
pub struct Dock {
    // Vector of strings representing the two distinct food items.
    food: Vec<String>,
    /// File to print to
    file_arc: Arc<Mutex<File>>,
    /// True if writing to file
    file_bool: bool
}

/// Implementation of the dock struct used to add additional functionality.
impl Dock {
    /// Dock constructor that has no arguments. Initializes an empty String vector.
    ///
    /// # Arguments
    /// 
    /// * 'file_arc' - File to write to
    /// * 'file_bool' - True if writing to file
    /// 
    /// # Return Value
    ///
    /// * `Dock` - Dock Struct
    pub fn new(file_arc: Arc<Mutex<File>>, file_bool: bool) -> Dock {
        Dock {
            food: Vec::new(),
            file_arc,
            file_bool
        }
    }

    /// This function places food on the dock only if there is enough room (Can't place more
    /// than two items at one time).
    ///
    /// # Arguments
    ///
    /// * `self` - mutable reference to the current object.
    /// * `meal` - String representation of the meal to be placed on the dock.
    ///
    /// # Return Value
    ///
    /// True if the food was successfully placed, false otherwise.
    pub fn place_food(&mut self, meal: String) -> bool {
        let rtn: bool;
        if self.food.len() < 2 {
            self.print_or_write("         Putting ".to_string() + &meal + " on dock.");
            self.food.push(meal);
            rtn = true;
        } else {
            self.print_or_write("\nCan't place food. Dock is full!\n".to_string());
            rtn = false;
        }
        rtn
    }

    /// This function will clear the vector string containing the supplies once they
    /// are picked up by the miners. It will notify the miners if there is no food on the dock
    /// or if there is only one item on the dock.
    ///
    /// # Arguments
    ///
    /// * `self` - mutable reference to the current object.
    pub fn take_food(&mut self) {
        if self.food.len() == 2 {
            self.food.clear();
        } else if self.food.len() == 0 {
            self.print_or_write("\nCan't take food. Dock is empty!\n".to_string());
        } else if self.food.len() == 1 {
            self.print_or_write("\nDock only has one food.\n".to_string());
        } else {
            self.print_or_write("\nDock is broken. Get it together\n".to_string());
        }
    }

    /// String representation of the supplies placed on the dock.
    ///
    /// # Arguments
    ///
    /// * `self` - mutable reference to the current object.
    ///
    /// # Return Value
    ///
    /// * `rtn` - string representation of the supplies.
    pub fn to_string(&mut self) -> String {
        let mut rtn = "".to_string();

        for f in &self.food {
            rtn += f;
            rtn += " and ";
        }

        rtn.truncate(rtn.len() - 5);
        rtn
    }

    /// Writes to file if boolean is set to true. Prints to console if not.
    /// 
    /// * 'pstr' - String to print
    fn print_or_write(&self, pstr: String) {
        if self.file_bool {
            let p = pstr + "\n";
            let file = &mut *self.file_arc.lock().unwrap();
            file.write_all(p.as_bytes()).expect("Error writing to file");
        } else {
            println!("{}", pstr);
        }
    }
}