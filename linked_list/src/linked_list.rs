// Implementing a singly linked list using Rust

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: i32,
}

impl<T> LinkedList<T> {

    // Methods to instantiate a new list.
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn from(vector: Vec<T>) -> LinkedList<T> {
        let mut new_list: LinkedList<T> = LinkedList::new();
        for item in vector {
            new_list.push_back(item);
        }

        return new_list;
    }

    pub fn is_empty(&self) -> bool {
        return self.length == 0;
    }

    pub fn push_front(&mut self, data: T) {
        if self.length == 0 {
            let new_node = Rc::new(RefCell::new(Node {
                data: data,
                next: None,
            }));

            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            self.length = 1;
            return;
        }

        let new_node = Rc::new(RefCell::new(Node {
            data: data,
            next: Some(Rc::clone(self.head.as_ref().unwrap())),
        }));

        self.head = Some(Rc::clone(&new_node));
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                let node_to_pop = (*node).borrow_mut();
                if let Some(new_head) = node_to_pop.next.as_ref() {
                    self.head = Some(Rc::clone(new_head));
                }
                else {
                    self.head = None;
                    self.tail = None;
                }
                self.length -= 1;

                // Now there should be no references to the node
                drop(node_to_pop);
                let unwrapped_result = Rc::try_unwrap(node).ok().unwrap().into_inner();
                return Some(unwrapped_result.data);
            }
        }
    }

    pub fn pop_all(&mut self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        while let Some(val) = self.pop_front() {
            result.push(val);
        }

        return result;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data: data,
            next: None,
        }));

        match mem::replace(&mut self.tail, None) {
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
                self.length = 1;
            }
            Some(node) => {
                self.tail = Some(Rc::clone(&new_node));
                self.length += 1;
                (*node).borrow_mut().next = Some(Rc::clone(&new_node));
            }
        }
    }

    pub fn append_back(&mut self, other: &mut LinkedList<T>) {
        if other.is_empty() {
            // The other list is empty, do nothing
            return;
        }

        match mem::replace(&mut self.tail, None) {
            None => {
                // The current list is empty, set head and tail to the other lists head and tail.
                self.head = Some(Rc::clone(other.head.as_ref().unwrap()));
                self.tail = Some(Rc::clone(other.tail.as_ref().unwrap()));
                self.length = other.length;
            }
            Some(node) => {
                // Both the lists are non-empty, join them.
                self.tail = Some(Rc::clone(other.tail.as_ref().unwrap()));
                (*node).borrow_mut().next = Some(Rc::clone(other.head.as_ref().unwrap()));
                self.length += other.length;
            }
        }

        // Clear the other list.
        other.head = None;
        other.tail = None;
        other.length = 0;
    }
}