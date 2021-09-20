use myvec::MyVec;
use std::mem::size_of;

fn main() {
    let mut vec: MyVec<&str> = MyVec::new();

    for _ in 0..100 {
        vec.push("haha");
    }
    let a = vec.get(0).unwrap() as *const _;
    let b = vec.get(1).unwrap() as *const _;

    println!("The items are {} byte(s) apart", b as usize - a as usize);
    println!("{}", size_of::<MyVec<&str>>());
}
