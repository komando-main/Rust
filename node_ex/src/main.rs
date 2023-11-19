use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None, prev: None }))
    }
    fn set_node(&mut self, value: i32) {
        let new_node = Node::new(value);
        if self.next.is_some() {
            self.prev = self.next.clone();
        }
        self.next = Some(new_node);
    }
}

fn main() {
    let mut node = Node::new(1);
    
    node.set_node(2);


}