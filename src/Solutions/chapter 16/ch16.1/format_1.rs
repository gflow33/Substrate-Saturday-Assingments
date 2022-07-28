/* 
fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!(__);
    assert_eq!(s, "hello, world!");
}
*/
fn main() {
    let s1 = "hello,";
    /* Fill in the blank */
    let s = format!("{} {}", s1, "world!" );
    assert_eq!(s, "hello, world!");
}
