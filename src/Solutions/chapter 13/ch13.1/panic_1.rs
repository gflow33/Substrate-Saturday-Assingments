/* 
// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        __
     }

    println!("Excercise Failed if printing out this line!");
}

fn main() {
    drink(__);

    println!("Excercise Failed if printing out this line!");
}
*/


// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        
    } else {
        panic!("Beverage not lemonde!");
        println!("Excercise Failed if printing out this line!");
    }

   
}

fn main() {
    drink("lemonade");

    //println!("Excercise Failed if printing out this line!");
}
