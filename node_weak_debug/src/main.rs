use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]  // Debug 트레이트를 파생
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        println!("Creating node: {:#?}", value);
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            previous: None,
        }))
    }

    fn link_node(prev: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let new_node = Node::new(prev.borrow().value + 1);
        prev.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node.borrow_mut().previous = Some(Rc::downgrade(prev));
        new_node
    }

    fn print_nodes(start: &Rc<RefCell<Node>>) {
        let mut current = Some(Rc::clone(start));
        while let Some(node) = current {
            // 구조체의 전체 내용을 출력
            println!("Visiting node: {:#?}", node.borrow());
            current = node.borrow().next.clone();
        }
    }
}

fn main() {
    let start = Node::new(0);
    let mut current = Rc::clone(&start);

    // 노드 연결
    for _ in 0..5 {
        current = Node::link_node(&current);
    }
    println!("\n\n current {:#?} \n\n", current);

    // 노드 출력
    println!("\nPrinting all nodes:");
    Node::print_nodes(&start);
}
