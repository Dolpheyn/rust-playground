use std::sync::Arc;
use std::thread;

fn main() {
    // Arc: A thread-safe reference-counting pointer. 'Arc' stands for
    // 'Atomically Reference Counted'.
    //
    // i.e. It's a smart pointer that can count how many pointers/variables
    // are currently pointed at it.
    let val = Arc::new(0);

    for i in 0..10 {
        // The last Arc is dropped at the end of the closure in
        // the spawned thread, so we need to clone a new Arc.
        //
        // The cloned value is the address pointed to the inner data.
        //
        // +------+-------+        +-------+---+
        // | val  | ptr0  | -----> | inner | 0 |
        // +------+-------+        +-------+---+
        //                            ^
        // **clone**                  |
        //                            |
        // +-------------+-------+    | // still points to the same inner
        // | cloned_val  | ptr0  | ----
        // +-------------+-------+
        //
        //  Arc calls `self.inner().strong.fetch_add(1, Relaxed);`
        //                    ^       ^
        //                    |       |_ AtomicUsize
        //                    |_ &ArcInner
        //
        //
        //  i.e. It adds 1 to the strong count
        //
        let val = Arc::clone(&val);

        thread::spawn(move || {
            println!(
                "Loop {:?}: Value: {:?} / Active pointers: {}",
                i,
                *val + i,
                Arc::strong_count(&val)
            );
            // ------ Cloned Arc is dropped here, decreasing strong count by 1
        });
    }

    // Output:
    //
    // Loop 1: Value: 1 / Active pointers: 5
    // Loop 2: Value: 2 / Active pointers: 4
    // Loop 0: Value: 0 / Active pointers: 3
    // Loop 3: Value: 3 / Active pointers: 4
    // Loop 4: Value: 4 / Active pointers: 3
    // Loop 5: Value: 5 / Active pointers: 3
    // Loop 6: Value: 6 / Active pointers: 3
    // Loop 7: Value: 7 / Active pointers: 3
    // Loop 8: Value: 8 / Active pointers: 3
    // Loop %
    //
    // Not all loops make it because we didn't `wait` on the main thread.
    // I think. xD
}
