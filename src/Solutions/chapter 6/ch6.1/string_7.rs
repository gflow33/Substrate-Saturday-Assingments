
// Fix error with at least two solutions
//fn main() {
    //let s = "hello, world";
    //greetings(s)
//}

//fn greetings(s: String) {
    //println!("{}",s)
//}

// Fix error with at least two solutions
fn main() {
    let s = "hello, world".to_string();
    let p = String::from("hello, world")

    greetings(s)
    greetings(p)
}

fn greetings(s: String) {
    println!("{}",s)
}
