//fn main() {
  //  assert!(0.1+0.2==0.3);

   // println!("Success!");
//}

fn main() {
    assert!(0.1_f32 + 0.2 as f32 == 0.3_f32);
    let x = 0.1 + 0.2;
    println!("{}", x);
    assert!(0.1 + 0.2 == 0.30000000000000004);
    println!("Success!");
}
