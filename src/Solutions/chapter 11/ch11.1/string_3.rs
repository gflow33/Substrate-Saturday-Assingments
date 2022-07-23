/* 
// Question: how many heap allocations are happend here ?
// Your answer: 
fn main() {  
    // Create a String type based on `&str`
    // the type of string literals is `&str`
   let s: String = String::from("hello, world!");

   // create a slice point to String `s`
   let slice: &str = &s;

   // create a String type based on the recently created slice
   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!")
}
*/



// Question: how many heap allocations are happend here ?
// Your answer: 2
fn main() {  
    // Create a String type based on `&str`
    // the type of string literals is `&str`
    let a = "hello, world!";
    let s = a.to_string();
   //let s: String = String::from("hello, world!");

   // create a slice point to String `s`
   let slice: &str = &s[0..];
   //let slice: &str = &s;

   // create a String type based on the recently created slice
   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!")
}

