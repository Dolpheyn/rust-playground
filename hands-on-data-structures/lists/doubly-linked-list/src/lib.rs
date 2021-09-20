use std::{cell::RefCell, rc::Rc};

// Node of data with type T inside a mutable memory location with dynamically(i.e. at runtime)
// checked borrow rules `RefCell` wrapped inside a single-threaded reference-counting pointer `Rc`.
//
// i.e. A single-threaded reference-counting pointer `Rc` that points to a mutable memory location
// with dynamically(i.e. at runtime) checked borrow rules `RefCell` that holds a `Node` with type
// T.
//
type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    item: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        Node {
            item,
            prev: None,
            next: None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    len: usize,
}

pub struct ListIterator<T> {
    current: Option<Link<T>>,
}

impl<T> ListIterator<T> {
    fn new(current: Option<Link<T>>) -> ListIterator<T> {
        ListIterator { current }
    }
}

// Turns DoublyLinkedList into ListIterator
impl<T> IntoIterator for DoublyLinkedList<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}

impl<T> Iterator for ListIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = None;

        // If the current Node is not None, return a cloned item T inside the Node.
        // Then, assign pointer to the next Node to `self.current`.
        self.current = match self.current {
            Some(ref current) => {
                let current = current.borrow();
                ret = Some(current.item.clone());

                current.next.clone()
            }
            None => None,
        };

        ret
    }
}

impl<T> DoubleEndedIterator for ListIterator<T>
where
    T: Clone,
{
    // Same with `next()` but backwards
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut ret = None;

        self.current = match self.current {
            Some(ref current) => {
                let current = current.borrow();
                ret = Some(current.item.clone());

                current.prev.clone()
            }
            None => None,
        };

        ret
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn append(&mut self, item: T) {
        let new = Rc::new(RefCell::new(Node::new(item)));

        // If the list is not empty (i.e. self.tail is not `None`), assign a pointer that points to
        // the new item to the old tail's `next`
        //
        // (1)
        // new
        // ...rest <--> old_tail
        //
        // (2) - Old tail's next is new
        // ...rest <--> old_tail --> new
        //
        // (3) - New's prev is old tail
        // ...rest <--> old_tail <--> new
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => {
                self.head = Some(new.clone());
            }
        }

        self.tail = Some(new);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn append_1() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        list.append(1);
        assert_eq!(list.len(), 1);

        let head = list.head;
        let tail = list.tail;

        assert!(head.is_some());
        assert!(tail.is_some());
    }

    #[test]
    fn append_2() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        let head = list.head;
        let tail = list.tail;

        assert!(head.is_some());
        assert!(tail.is_some());
    }

    #[test]
    fn iterator() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        let size = 10;

        for i in 0..size {
            list.append(i);
        }

        let mut count = 0;
        for _ in list {
            count += 1;
        }

        assert_eq!(count, size);
    }

    #[test]
    fn rev_iterator() {
        let mut list: DoublyLinkedList<usize> = DoublyLinkedList::new();
        let size = 10;

        for i in 0..size {
            list.append(i);
        }

        let mut iter = list.into_iter();

        for _ in 0..(size - 1) {
            iter.next();
        }

        let mut count = 0;
        for _ in iter.rev() {
            count += 1;
        }

        assert_eq!(count, size);
    }
}
