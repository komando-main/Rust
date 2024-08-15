use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    num: i32,
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Rc<RefCell<Node>>>,
}

/*
use std::rc::Rc;: Rc는 "Reference Counted"의 약자로, 여러 개의 소유자가 있을 수 있는 힙 할당된 값을 참조할 수 있게 합니다.
use std::cell::RefCell;: RefCell은 런타임에 가변성을 제공하는 타입으로, Rc와 함께 사용하여 내부 가변성을 처리할 수 있게 합니다.
struct Node: 이 구조체는 이중 연결 리스트의 노드를 나타냅니다. 각 노드는 num 값을 가지며, next와 previous 포인터는
Rc<RefCell<Node>> 타입으로 연결 리스트의 다음과 이전 노드를 가리킵니다.
*/
impl Node {
    fn new() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            num: 0,
            next: None,
            previous: None,
        }))
    }
/*
fn new() -> Rc<RefCell<Node>>: 새로운 노드를 생성합니다.
Rc::new로 RefCell을 감싸서 Node를 생성합니다.
이렇게 하면 여러 참조자가 이 노드를 공유할 수 있게 됩니다.
*/
    fn link_node(table: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let create_table = Rc::new(RefCell::new(Node {
            num: table.borrow().num + 1,
            next: None,
            previous: Some(Rc::clone(&table)),
        }));
        table.borrow_mut().next = Some(Rc::clone(&create_table));
        create_table
    }
    /*
    fn link_node(table: Rc<RefCell<Node>>) -> Rc<RefCell<Node>>: 주어진 노드 table에 새로운 노드를 연결합니다.
        let produce_table = Rc::new(RefCell::new(Node {...}));: 새로운 노드를 생성하고 table의 num 값에 1을 더한 값을 가집니다.
        previous 포인터는 table을 가리킵니다.
        table.borrow_mut().next = Some(Rc::clone(&produce_table));: RefCell을 통해 table의 next 포인터를 변경합니다.
        Rc::clone을 사용하여 produce_table을 복사합니다. 이는 참조 카운트를 증가시킵니다.
    */

    fn run_table(data: Rc<RefCell<Node>>, num: i32) -> Rc<RefCell<Node>> {
        let mut current = Rc::clone(&data);
        for _ in 0..num {
            current = Node::link_node(current);
        }
        current
    }
/*
fn run_table(data: Rc<RefCell<Node>>, num: i32) -> Rc<RefCell<Node>>: num 개의 노드를 생성하여 연결합니다.
    let mut current = Rc::clone(&data);: 초기 노드를 current에 복사합니다.
    for _ in 0..num: 루프를 돌며 num 개의 노드를 생성하고 연결합니다.
    current = Node::link_node(current);: 현재 노드에 새 노드를 연결하고 current를 업데이트합니다.
*/
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
/*
fn all_node_check(all_table: Rc<RefCell<Node>>): 연결 리스트의 노드를 출력합니다.

    let mut current = Some(Rc::clone(&all_table));: 초기 노드를 current에 복사합니다.
    while let Some(node) = current: current가 Some인 동안 루프를 실행합니다.
    println!: 현재 노드의 num 값을 출력합니다.
    current = node.borrow().next.clone();: 다음 노드로 이동합니다.
*/
    fn remove_table(mut table: Rc<RefCell<Node>>) {
        loop {
            let previous_node = table.borrow().previous.clone();
            if let Some(ref prev) = previous_node { //ref: 소유권 규칙에 위배대지 않도록 대여할때 사용하는 키워드 입니다.
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
/*
fn remove_table(mut table: Rc<RefCell<Node>>): 연결 리스트의 노드를 제거합니다.
    let previous_node = table.borrow().previous.clone();: 이전 노드를 가져옵니다.
    if let Some(ref prev) = previous_node: 이전 노드가 있는 경우, prev의 next 포인터를 None으로 설정합니다.
    println!("remove table->num: {}", table.borrow().num);: 현재 노드의 num 값을 출력합니다.
    let next_node = table.borrow().next.clone();: 다음 노드를 가져옵니다.
    if let Some(next) = next_node: 다음 노드가 있는 경우, table을 다음 노드로 업데이트합니다.
    else { break; }: 더 이상 다음 노드가 없으면 루프를 종료합니다.
*/
    fn add_data(data: Rc<RefCell<Node>>, num: i32) {
        let mut data = Rc::clone(&data);
        for _ in 0..3 {
            let prev = data.borrow().previous.as_ref().unwrap().clone();
            drop(data);  // 현재 참조를 명시적으로 드롭하여 참조 해제
            data = prev;
        }

        let up = Rc::clone(data.borrow().previous.as_ref().unwrap());
        let down = Rc::clone(&data);
        let add_data = Rc::new(RefCell::new(Node {
            num,
            next: Some(Rc::clone(&down)),
            previous: Some(Rc::clone(&up)),
        }));

        up.borrow_mut().next = Some(Rc::clone(&add_data));
        down.borrow_mut().previous = Some(Rc::clone(&add_data));
    }
}
/*
fn add_data(data: Rc<RefCell<Node>>, num: i32): 특정 위치에 새로운 노드를 추가합니다.

    let mut data = Rc::clone(&data);: 초기 노드를 data에 복사합니다.
    for _ in 0..3: 3단계 이전 노드로 이동합니다.
    let prev = data.borrow().previous.as_ref().unwrap().clone();: 이전 노드를 가져옵니다.
    drop(data);: 현재 노드의 참조를 드롭합니다.
    data = prev;: data를 이전 노드로 업데이트합니다.
    let up = Rc::clone(data.borrow().previous.as_ref().unwrap());: 상위 노드를 가져옵니다.
    let down = Rc::clone(&data);: 하위 노드를 data로 설정합니다.
    let add_data = Rc::new(RefCell::new(Node { ... }));: 새로운 노드를 생성합니다.
    up.borrow_mut().next = Some(Rc::clone(&add_data));: 상위 노드의 next 포인터를 새로운 노드로 설정합니다.
    down.borrow_mut().previous = Some(Rc::clone(&add_data));: 하위 노드의 previous 포인터를 새로운 노드로 설정합니다.
*/
fn main() {
    let start = Node::new();
    let current = Node::run_table(Rc::clone(&start), 10);
    // let end = Rc::clone(&current);
    start.borrow_mut().previous = Some(Rc::clone(&current));
    current.borrow_mut().next = Some(Rc::clone(&start));
    // let loop_node = Rc::clone(&start);
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
/*
fn main(): 프로그램의 진입점입니다.

    let start = Node::new();: 새로운 시작 노드를 생성합니다.
    let current = Node::run_table(Rc::clone(&start), 10);: 10개의 노드를 생성하여 연결합니다.
    let end = Rc::clone(&current);: 마지막 노드를 end에 복사
*/