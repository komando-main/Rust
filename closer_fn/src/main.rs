fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    // let add = |a, b| a + b;
    // let add: Box<_> = Box::new(add);
    let name = "음! 좋아";//바인드가 힙영으로 넘거가면 사용 불가!
    let add = Box::new(move |a, b| {
        println!("더하기 {}", name);//힙영역에 생성 되어 있기 때문에 move 키워드를 해줘야 넘어간다
        a + b
    });
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, mul));
}