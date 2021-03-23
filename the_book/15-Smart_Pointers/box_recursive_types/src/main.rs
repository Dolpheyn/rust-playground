#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/**
 * Error: recursive type has infinite size
 *
 * When compiling, a rust defines enum's size as the variant that takes the most space. In List,
 * the variant that takes the most space is `Cons(i32, List)`.
 *
 * In order to determine how much space `Cons` takes, the value of i32 + the value of `List` must
 * be calculated.
 *
 * Because List is recursive, this recursive size defining operation will theoretically go
 * infinite.

enum List {
    Cons(i32, List),
    Nil,
}
**/

fn main() {
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list); // Cons(1, Cons(2, Cons(3, Nil)))
}
