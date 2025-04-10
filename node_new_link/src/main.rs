use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    num: i32,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            num: 0,
            next: None,
            previous: None,
        }))
    }

    fn link_node(current: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let new_node = Rc::new(RefCell::new(Node {
            num: current.borrow().num + 1,
            next: None,
            previous: Some(Rc::downgrade(&current)),
        }));

        current.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node
    }

    fn run_table(start: Rc<RefCell<Node>>, count: i32) -> Rc<RefCell<Node>> {
        let mut current = start;
        for _ in 0..count {
            current = Self::link_node(current);
        }
        current
    }

    fn all_node_check(start: Rc<RefCell<Node>>) {
        let mut current = Some(Rc::clone(&start));
        let mut count = 0;

        while let Some(node) = current {
            if count > 23 {
                println!("counts: {}, cycles: {}\nbreak while let!!!", count, count / 12);
                break;
            }

            println!("counts: {}, num: {}", count, node.borrow().num);
            current = node.borrow().next.clone();
            count += 1;
        }
    }

    fn remove_table(current: Rc<RefCell<Node>>) {
        let mut current = current;
    
        // 순방향 순회
        loop {
            let next = {
                let current_borrow = current.borrow();
                current_borrow.next.clone()
            };
    
            match next {
                Some(next_node) => current = next_node,
                None => break,
            }
        }
    
        // 역방향 순회
        let mut target = Rc::clone(&current);
    
        loop {
            let previous = {
                let target_borrow = target.borrow();
                target_borrow.previous.clone()
            };
    
            match previous {
                Some(prev_weak) => {
                    if let Some(prev_node) = prev_weak.upgrade() {
                        target = prev_node;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }
    fn add_data(current: Rc<RefCell<Node>>, num: i32) {
        // 3칸 위의 노드 찾기
        let mut target = Rc::clone(&current);
        for _ in 0..3 {
            let prev = {
                let target_borrow = target.borrow();
                target_borrow.previous.as_ref()
                    .expect("No previous node found")
                    .upgrade()
                    .expect("Previous node missing")
            };
            target = prev;
        }

        let up = target.borrow().previous.as_ref()
            .and_then(|w| w.upgrade())
            .expect("No upper node");

        let down = Rc::clone(&target);

        let inserted = Rc::new(RefCell::new(Node {
            num,
            next: Some(Rc::clone(&down)),
            previous: Some(Rc::downgrade(&up)),
        }));

        up.borrow_mut().next = Some(Rc::clone(&inserted));
        down.borrow_mut().previous = Some(Rc::downgrade(&inserted));
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
    let insert_value = 10000;
    Node::add_data(Rc::clone(&loop_node), insert_value);

    println!("\n▶ All Node Check:\n");
    Node::all_node_check(Rc::clone(&loop_node));

    println!("\n▶ Removing Nodes:\n");
    Node::remove_table(Rc::clone(&loop_node));

    println!("\n▶ Done!");
}
