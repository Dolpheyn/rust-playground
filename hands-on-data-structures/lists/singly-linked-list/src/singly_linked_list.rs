use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::Rc,
};

// Node of data with type T inside a mutable memory location with dynamically(i.e. at runtime)
// checked borrow rules `RefCell` wrapped inside a single-threaded reference-counting pointer `Rc`.
//
// i.e. A single-threaded reference-counting pointer `Rc` that points to a mutable memory location
// with dynamically(i.e. at runtime) checked borrow rules `RefCell` that holds a `Node` with type
// T.
//
type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Link<T> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => (*old_tail).borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.len += 1;
        self.tail = Some(new_node);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Return a reference to the tail node if its not None, else return None
    pub fn peek(&self) -> Option<Ref<Node<T>>> {
        self.tail.as_ref().map(|link| link.borrow())
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            if let Some(next) = (*head).borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail = None;
            }

            let val = Rc::try_unwrap(head).ok().expect("blaaa").into_inner().value;
            Some(val)
        } else {
            None
        }
    }
}
