
#![allow(unused_mut, dead_code)]
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None}))
    }
    fn set_node(&mut self, value: i32) {
        let new_node = Node::new(value);
        self.next = new_node;
    }
}

fn main() {
    let mut node = Node::new(1);
    node.borrow_mut().set_node(2);
    node.borrow_mut().set_node(3);
    node.borrow_mut().set_node(4);

    println!("{:#?}", node);
}