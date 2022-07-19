/* 
// Implement the generic function below.
fn sum

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}
*/

// Implement the generic function below.
use std::ops::Add;
fn sum<T: std::ops::Add + Add<Output = T>>(s: T, v: T) -> T{
    s + v
    
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}

