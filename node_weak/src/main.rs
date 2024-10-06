use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    num: i32,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Weak<RefCell<Node>>>,  // Weak<T> 사용
}

impl Node {
    fn new() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            num: 0,
            next: None,
            previous: None,
        }))
    }

    fn link_node(table: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let create_table = Rc::new(RefCell::new(Node {
            num: table.borrow().num + 1,
            next: None,
            previous: Some(Rc::downgrade(&table)),  // 약한 참조로 순환 참조 방지
        }));
        table.borrow_mut().next = Some(Rc::clone(&create_table));
        create_table
    }

    fn run_table(data: Rc<RefCell<Node>>, num: i32) -> Rc<RefCell<Node>> {
        let mut current = Rc::clone(&data);
        for _ in 0..num {
            current = Node::link_node(current);
        }
        current
    }

    fn all_node_check(all_table: Rc<RefCell<Node>>) {
        let mut current = Some(Rc::clone(&all_table));
        let mut count = 0;

        while let Some(node) = current {
            if count > 23 {
                println!("counts: {}, cycles: {}\nbreak while let!!!", count, count / 12);
                break;
            }

            println!("counts: {}, all_table->num: {}", count, node.borrow().num);
            current = node.borrow().next.clone();
            count += 1;
        }
    }

    fn remove_table(mut table: Rc<RefCell<Node>>) {
        loop {
            let previous_node = table.borrow().previous.clone();
            if let Some(prev) = previous_node.and_then(|weak| weak.upgrade()) {  // Weak를 강한 참조로 업그레이드
                prev.borrow_mut().next = None;
            }

            println!("remove table.borrow().num: {}", table.borrow().num);

            let next_node = table.borrow().next.clone();
            if let Some(next) = next_node {
                table = Rc::clone(&next);
            } else {
                break;
            }
        }
    }

    fn add_data(data: Rc<RefCell<Node>>, num: i32) {
        let mut data = Rc::clone(&data);
        for _ in 0..3 {
            let prev = data.borrow().previous.as_ref().unwrap().upgrade().unwrap();
            drop(data);  // 현재 참조를 명시적으로 드롭하여 참조 해제
            data = prev;
        }

        let up = data.borrow().previous.as_ref().unwrap().upgrade().unwrap();
        let down = Rc::clone(&data);
        let add_data = Rc::new(RefCell::new(Node {
            num,
            next: Some(Rc::clone(&down)),
            previous: Some(Rc::downgrade(&up)),
        }));

        up.borrow_mut().next = Some(Rc::clone(&add_data));
        down.borrow_mut().previous = Some(Rc::downgrade(&add_data));
    }
}

fn main() {
    let start = Node::new();
    let current = Node::run_table(Rc::clone(&start), 10);
    start.borrow_mut().previous = Some(Rc::downgrade(&current));
    current.borrow_mut().next = Some(Rc::clone(&start));
    let loop_node = start;
    let num = 10000;
    Node::add_data(Rc::clone(&loop_node), num);
    println!("\n");
    Node::all_node_check(Rc::clone(&loop_node));
    println!("\n");
    Node::remove_table(Rc::clone(&loop_node));
    println!("\n");
    println!("여기까진 오긴 하니?\n");
    println!("프로그램 끝내기!!");
}
