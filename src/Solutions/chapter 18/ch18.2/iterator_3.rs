/* /* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let v1 = vec![1, 2];

    assert_eq!(v1.next(), __);
    assert_eq!(v1.next(), __);
    assert_eq!(v1.next(), __);
}
*/

/* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let v1 = vec![1, 2];
    let mut v2 = v1.into_iter();
    assert_eq!(v2.next(), Some(1));
    assert_eq!(v2.next(), Some(2));
    assert_eq!(v2.next(), None);
}
