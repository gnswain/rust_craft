
pub struct Dock {
    food: Vec<String>
}

impl Dock {
    pub const fn new() -> Self {
        Dock {
            food: Vec::new()
        }
    }

    pub fn place_food(&mut self, meal: String) -> bool {
        let rtn: bool;
        if self.food.len() < 2 {
            println!("Putting {} on dock.", meal);
            self.food.push(meal);
            rtn = true;
        } else {
            println!("\nCan't place food. Dock is full!\n");
            rtn = false;
        }
        rtn
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