use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    num: i32,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Weak<RefCell<Node>>>, //Weak는 Rc없이 단독 사용 불가
}

trait FnNode {
    fn new() -> Rc<RefCell<Self>>;
    fn link_node(current: Rc<RefCell<Self>>) -> Rc<RefCell<Self>>;
    fn run_table(start: Rc<RefCell<Self>>, count: i32) -> Rc<RefCell<Self>>;
    fn all_node_check(start: Rc<RefCell<Self>>);
    fn add_data(current: Rc<RefCell<Self>>, num: i32);
    fn remove_table(node: Rc<RefCell<Self>>);
}

impl FnNode for Node {
    fn new() -> Rc<RefCell<Self>> {//새로운 노드 만들기
        Rc::new(RefCell::new(Self {
            num: 0,
            next: None,
            previous: None
        }))
    }
    fn link_node(current: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let new_node = Rc::new(RefCell::new(Self {//기존노드에 새로운노드 추가 하기
            num: current.borrow().num + 1,
            next: None,
            previous: Some(Rc::downgrade(&current)),//기존 노드의 주소를 다운그래이드 하여 저장 하기 다운그래이드 하는 이유는 무한루프에 빠지지 않기 위해서
        }));

        current.borrow_mut().next = Some(Rc::clone(&new_node));//기존 노드의 세로운 노드 주소 복재하여 저장하기
        new_node
    }
    fn run_table(start: Rc<RefCell<Self>>, count: i32) -> Rc<RefCell<Self>> {
        (0..count).fold(start, |current, _| Self::link_node(current))//포문대신하여 자동으로 몇개를 생성할지를 결정하여 자동생성하기
    }
    fn all_node_check(start: Rc<RefCell<Self>>) {
        let mut current = Some(start);//현재 노드 주소를 저장하기 위한 변수
        let mut count = 0;//노드의 개수를 세기 위한 변수

        while let Some(node_rc) = current {
            if count > 23 {
                println!("counts: {}, cycles: {}\nbreak while let!!!", count, count / 12);//노드의 개수가 23개 이상이 되면 무한루프에 빠지지 않도록 break 하기
                break;
            }

            let node = node_rc.borrow();
            println!("count: {}, num: {}", count, node.num);//현재 노드의 개수와 num값을 출력하기
            current = node.next.clone();
            count += 1;
        }
    }
    fn add_data(current: Rc<RefCell<Self>>, num: i32) {
        let mut target = Rc::clone(&current);
        for _ in 0..3 {//3번 반복하여 현재 노드의 주소를 저장하기
            let prev = {
                let target_ref = target.borrow();
                target_ref.previous.as_ref().expect("No date").upgrade().expect("No date")//Weak를 Rc로 업그레이드 하기
            };
            target = prev;//이전 노드의 주소를 저장하기
        }

        let up = {//불리시키기위한 작업
            let target_ref = target.borrow();
            target_ref.previous.as_ref().and_then(|w|w.upgrade()).expect("No date")
        };

        let inerted = Rc::new(RefCell::new(Self{//새로 노드를 샐성하여 반호 임력
            num,
            next: Some(Rc::clone(&target)),
            previous: Some(Rc::downgrade(&up)),
        }));
        up.borrow_mut().next = Some(Rc::clone(&inerted));//불리시킨 업의 넥스트에 클로그로 연결
        target.borrow_mut().previous = Some(Rc::downgrade(&inerted));//불리시킨 타겟에 프리비우스 다운그래이드로 연결
    }
    fn remove_table(node: Rc<RefCell<Self>>) {
        let mut current = Some(node);

        while let Some(n) = current {
            {
                let node_ref = n.borrow_mut();
                if let Some(prev_rc) = node_ref.previous.as_ref().and_then(|w|w.upgrade()) {
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

    start.borrow_mut().previous = Some(Rc::downgrade(&end));
    end.borrow_mut().next = Some(Rc::clone(&start));

    let loop_node = Rc::clone(&start);
    Node::add_data(Rc::clone(&loop_node), 10000);

    println!("\n▶ All Node Check:\n");
    Node::all_node_check(Rc::clone(&loop_node));

    end.borrow_mut().next = None;
    start.borrow_mut().previous = None;

    println!("\n▶ Removing Nodes:\n");
    Node::remove_table(loop_node);

    println!("\n▶ Done!");
}