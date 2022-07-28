/* #![allow(unused)]
fn main() {
// in lib.rs

// FILL in the blanks and FIX the errors
// You need make something public with `pub` to provide accessiblity for outside code `fn eat_at_restaurant()`
mod front_of_house {
    /* ...snip... */
}

pub fn eat_at_restaurant() {
    // call add_to_waitlist with **absolute path**:
    __.add_to_waitlist();

    // call with **relative path** 
     __.add_to_waitlist();
}
}
*/

#![allow(unused)]
fn main() {
// in lib.rs

// FILL in the blanks and FIX the errors
// You need make something public with `pub` to provide accessiblity for outside code `fn eat_at_restaurant()`
pub mod front_of_house {
    /* ...snip... */
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}

        pub fn complain() {} 
    }
}

pub fn eat_at_restaurant() {
    // call add_to_waitlist with **absolute path**:
    crate::front_of_house::hosting::add_to_waitlist();

    // call with **relative path** 
    front_of_house::hosting::add_to_waitlist();
}
}
