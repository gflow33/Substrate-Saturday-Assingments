// Fix the error with at least two solutions
//fn main() {
    //let s: Box<str> = "hello, world".into();
    //greetings(s)
//}

//fn greetings(s: &str) {
    //println!("{}",s)
//}

// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
    let t: Box<&str> = "hello, world".into();
    greetings(&t)
}

fn greetings(s: &str) {
    println!("{}",s)
}
