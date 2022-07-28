/* 
/* Fill in the blanks and Fix the errors */
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("__ months in a year.", 12);

    println!("Now __ will print!", Structure(3));
}
*/

/* Fill in the blanks and Fix the errors */
#[derive(Debug)]
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}
