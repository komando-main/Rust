#[derive(Clone, Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }

    fn add_data(&mut self, new_node: Node) {
        self.next = Some(Box::new(std::mem::replace(&mut *self, new_node)));
    }
}

fn print_all_nodes(all_nodes: &Node) {
    let mut current = all_nodes.clone();

    while let Some(node) = current.next.as_ref() {
        println!("{}", node.data);
        current = (**node).clone(); // 수정: 언박싱 후에 다시 언박싱하여 Node로 변환
    }
}

fn main() {
    let mut start = Node::new(1);
    let new_node = Node::new(2);

    start.add_data(new_node.clone());

    println!("All Nodes:");
    print_all_nodes(&start);
}
