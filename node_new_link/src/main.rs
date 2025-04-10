use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    num: i32,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            num: 0,
            next: None,
            previous: None,
        }))
    }

    fn link_node(current: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let new_node = Rc::new(RefCell::new(Self {
            num: current.borrow().num + 1,
            next: None,
            previous: Some(Rc::downgrade(&current)),
        }));

        current.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node
    }

    fn run_table(start: Rc<RefCell<Self>>, count: i32) -> Rc<RefCell<Self>> {
        (0..count).fold(start, |current, _| Self::link_node(current))
    }

    fn all_node_check(start: Rc<RefCell<Self>>) {
        let mut current = Some(start);
        let mut count = 0;

        while let Some(node_rc) = current {
            if count > 23 {
                println!("counts: {}, cycles: {}\nbreak while let!!!", count, count / 12);
                break;
            }

            let node = node_rc.borrow();
            println!("counts: {}, num: {}", count, node.num);
            current = node.next.clone();
            count += 1;
        }
    }

    fn add_data(current: Rc<RefCell<Self>>, num: i32) {
        let mut target = Rc::clone(&current);
        for _ in 0..3 {
            let prev = {
                let target_ref = target.borrow();
                target_ref.previous
                    .as_ref()
                    .expect("No previous node")
                    .upgrade()
                    .expect("Previous node dropped")
            };
            target = prev;
        }
    
        let up = {
            let target_ref = target.borrow();
            target_ref.previous
                .as_ref()
                .and_then(|w| w.upgrade())
                .expect("No upper node")
        };
    
        let inserted = Rc::new(RefCell::new(Self {
            num,
            next: Some(Rc::clone(&target)),
            previous: Some(Rc::downgrade(&up)),
        }));
    
        up.borrow_mut().next = Some(Rc::clone(&inserted));
        target.borrow_mut().previous = Some(Rc::downgrade(&inserted));
    }

    fn remove_table(node: Rc<RefCell<Self>>) {
        let mut current = Some(node);
    
        while let Some(n) = current {
            {
                let node_ref = n.borrow_mut();
                if let Some(prev_rc) = node_ref.previous.as_ref().and_then(|w| w.upgrade()) {
                    prev_rc.borrow_mut().next = None;
                }
                println!("remove table.borrow().num: {}", node_ref.num);
            }
    
            let next = {
                n.borrow().next.clone()
            };
            current = next;
        }
    }
}

fn main() {
    let start = Node::new();
    let end = Node::run_table(Rc::clone(&start), 10);

    // 순환 연결 구성
    start.borrow_mut().previous = Some(Rc::downgrade(&end));
    end.borrow_mut().next = Some(Rc::clone(&start));

    // 중간에 데이터 추가
    let loop_node = Rc::clone(&start);
    Node::add_data(Rc::clone(&loop_node), 10000);

    println!("\n▶ All Node Check:\n");
    Node::all_node_check(Rc::clone(&loop_node));

    // 💡 순환 제거 후 삭제
    end.borrow_mut().next = None;
    start.borrow_mut().previous = None;

    println!("\n▶ Removing Nodes:\n");
    Node::remove_table(loop_node);

    println!("\n▶ Done!");
}