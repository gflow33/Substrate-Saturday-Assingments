//fn main() {
    //assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
    //assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
//}


pub use crate::front_of_house::hosting;
fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
     assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
