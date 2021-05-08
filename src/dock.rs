
pub struct Dock {
    food: Vec<String>
}

impl Dock {
    pub const fn new() -> Self {
        Dock {
            food: Vec::new()
        }
    }

    pub fn place_food(&mut self, meal: String) {
        if self.food.len() < 2 {
            self.food.push(meal);
        } else {
            println!("\nCan't place food. Dock is full!\n");
        }
    }

    /// For the miners
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
}