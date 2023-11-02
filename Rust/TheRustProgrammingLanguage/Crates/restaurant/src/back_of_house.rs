// can make struct pub but fields will still be private
// have to also mark fields as pub
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// if we make enum public, all of its fields are also public
pub enum Appetizer {
    Soup, 
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order(); // path begins in parent module due to super
}

fn cook_order() {}