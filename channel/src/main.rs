use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("hi");
        for i in 1..5 {
            println!("{}", i);
            thread::sleep(Duration::from_secs(1));
        }

        tx.send(s).unwrap();
    });

    let recv = rx.recv().unwrap();
    println!("Got: {}", recv);
}
