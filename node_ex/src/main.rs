#![allow(dead_code, unused_imports)]

use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            next : None,
        }
    }
    fn set_node(self, value: i32) -> Node {
        let mut node = Node::new(value);
        node.next = Some(Box::new(self));
        node
    }

}

fn main() {
    let node = Node::new(0);
    println!("{:#?}", node);
    let mut node1 = node.set_node(1);
    for i in 1..=5{
        node1 = node1.set_node(i)
        
    }


    println!("{:#?}", node1.next);
    
    


    // println!("{}", node.borrow_mut().get_mut());
    
}