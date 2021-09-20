mod singly_linked_list;
use singly_linked_list::SinglyLinkedList;

use std::fmt::{Debug, Display};

fn peak<T>(list: &SinglyLinkedList<T>)
where
    T: Display + Debug,
{
    let tail = list.peek().unwrap();
    println!("Tail({}): {:?}", tail.value(), tail);
}

fn main() {
    let mut list = SinglyLinkedList::new();
    for i in 0..10 {
        list.append(i);
    }

    assert!(!list.is_empty());
    assert!(list.peek().is_some());

    peak(&list);

    for i in 0..list.len() {
        println!("{} {:?}", i, list.pop());
    }
}
