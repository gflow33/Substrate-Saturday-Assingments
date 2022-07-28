// in src/lib.rs

// IMPLEMENT...
// in src/lib.rs

mod front_of_house;
mod back_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}


// in src/back_of_house.rs

// IMPLEMENT...
// in src/back_of_house.rs

use crate::front_of_house;
pub fn fix_incorrect_order() {
    cook_order();
    front_of_house::serving::serve_order();
}

pub fn cook_order() {}


// in src/front_of_house/mod.rs

// IMPLEMENT...
pub mod hosting
pub mod serving


// in src/front_of_house/hosting.rs

// IMPLEMENT...
fn add_to_waitlist() {}
fn seat_at_table() -> String {
    String::from("sit down please")
}

// in src/front_of_house/serving.rs

// IMPLEMENT...
pub fn take_order() {}
pub fn serve_order() {}
pub fn take_payment() {}
fn complain() {}

