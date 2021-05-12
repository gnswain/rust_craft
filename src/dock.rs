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
    food: Vec<String>
}

/// Implementation of the dock struct used to add additional functionality.
impl Dock {
    /// Dock constructor that has no arguments. Initializes an empty String vector.
    ///
    /// # Return Value
    ///
    /// * `Dock` - Dock Struct
    pub const fn new() -> Self {
        Dock {
            food: Vec::new()
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
            println!("         Putting {} on dock.", meal);
            self.food.push(meal);
            rtn = true;
        } else {
            println!("\nCan't place food. Dock is full!\n");
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
            println!("\nCan't take food. Dock is empty!\n")
        } else if self.food.len() == 1 {
            println!("\nDock only has one food.\n");
        } else {
            println!("\nDock is broken. Get it together\n");
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
}