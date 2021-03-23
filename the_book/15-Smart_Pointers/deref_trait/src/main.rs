#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 7;
    let y = MyBox::new(x);

    // first we dereference MyBox(7) to get the value, which is a &i32.
    // then we use the dereference operator `*` to get the i32 value.
    assert_eq!(x, *y.deref());

    // The dereference operation in this one gets desugared into *(y.deref()),
    // which yields the same value as the *y.deref() above.
    assert_eq!(x, *y);

    println!("{:?} {:?}", x, y);

    let m = MyBox::new(String::from("Rust"));

    // # Deref Coercion
    // Here, the value of &MyBox<String> can be passed to `hello` because of something
    // called `Deref Coercion`.
    //
    // The type `String` can be inserted to a function that requires `&str` because
    // String implements Deref with Target str.
    //
    // Because `MyBox` implements a Deref into T, calling the deref() function gets us
    // &T, which in this case is a &String. So in order to pass String into the `hello`
    // function, we need to deref MyBox and deref again to get the String, which implements
    // deref to str, which we can then get &str using the deref on String.
    //
    // * All of the conversions happen at compile time, which means there's no runtime
    //   overhead.
    hello(&m);

    // Rust use the `*` operator on MyBox<String> to get String, then reference it:
    //   &(*m)
    // Which gets us &String, that can be coerced to turn into a &str too.
    hello(&(*m));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
