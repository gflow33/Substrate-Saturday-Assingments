/* // in src/main.rs

// FILL in the blank and FIX the errors
fn main() {
    assert_eq!(__, "sit down please");
    assert_eq!(__,"yummy yummy!");
}
*/
// in src/main.rs

// FILL in the blank and FIX the errors
mod front_of_house;
fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");
}

