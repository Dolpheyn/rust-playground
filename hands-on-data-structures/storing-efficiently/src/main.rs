use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new(String::from("Hello world"));
    println!("{:?}", a.as_ref() as *const _);

    let mut threads = vec![];
    for i in 0..2 {
        let c = a.clone();
        threads.push(thread::spawn(move || {
            println!("{:?}", c.as_ref() as *const _);
            println!("From thread {}: {}", i, Arc::strong_count(&c));
        }));
    }
    println!("From main: {}", Arc::strong_count(&a));
    drop(a);

    for child in threads {
        let _ = child.join();
    }

    // Output:
    //
    // 0x560f6af1ca20
    // From main: 3 <-- `a` and the two threads are counted
    // 0x560f6af1ca20
    // From thread 0: 2 <-- `a` is dropped, but the two threads are still holding
    // 0x560f6af1ca20
    // From thread 1: 1 <-- thread 0 ended its execution so it drops its own `c`
    //
    // Because Arc is thread-safe, the value is stored in the Heap and can be pointed to from other
    // threads. We can see that all the threads point to the same thing in memory (at addr
    // 0x560f6af1ca20)
}
