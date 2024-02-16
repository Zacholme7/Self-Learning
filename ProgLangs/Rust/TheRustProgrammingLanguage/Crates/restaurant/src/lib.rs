/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

// paths can take two forms
// absolute path: full path starting from teh crate root, starts with literal "crate"
// relative path: starts from the currnet module and uses "self" or "super"
// both followed by one or more identifiers separated by "::"

// everything is private to parten modules by default
// items in parent modules cant use private items inside child modules
// items in child modules can use items in ancestor modules
// define a module using mod

// can use "use" to bring things into scope
use ::crate::front_of_house::hosting;

// can use nested imports
use std::io::{self, Write};

// glob operator will bring all public items defined in a path into scope
use std::collections::*;

// can use "as" to alias
use std::fmt::Result;
use std::io::Result as IoResult;

mod front_of_house;
mod back_of_house;


fn deliver_order() {};

// part of api, so we mark it as pub
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // path with the use keyword
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");

    let order1 = back_of_house::Appetizer::Soup;
}