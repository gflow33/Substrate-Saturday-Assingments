/* // in lib.rs

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // FILL in the blank in tree ways
        //1. using keyword `super`
        //2. using absolute path
        __.serve_order();
    }

    fn cook_order() {}
}
*/

// in lib.rs

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // FILL in the blank in tree ways
        //1. using keyword `super`
        //2. using absolute path
        super::front_of_house::serving::serve_order();
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
