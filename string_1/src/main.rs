fn main() {
    let a = String::from("hello");
    let z = a.clone();
    let byte = z.bytes().nth(0);
    println!("{:?}", byte);
    match byte {
        Some(b) => {
                        println!("{}", b);
                        println!("{}", b as char);
                        },
        None => println!("문자열이 비어 있습니다."),
    }
    let mut b = a.chars();//[char:6] 배열 형태의 글짜 직접적으로는 쓸수 없다! 대신 디버그로는 확인할수 있다 사용 하려면 매치를 활용 해야 한다
    println!("a의 상태 {:?}", b);
    println!();
    let mut b1 = b.clone();
    println!("b1의 상태 {:?}", b1);
    match b1.nth(0) {
        Some(ch) => println!("match안 에서 h 를 넘김으로써 전채의 hello 에서 h 가 사용되면서 빠진다 {}", ch),
        None => println!("문자열이 비어 있습니다."),
    }
    println!("match 후 b1의 상태 {:?}", b1);
    println!();
    println!("match 전의 b 상태 {:?}", b);
    match b.nth(0) {
        Some(ch) => println!("match안 에서 h 를 넘김으로써 전채의 hello 에서 h 가 사용되면서 빠진다 {}", ch),
        None => println!("문자열이 비어 있습니다."),
    }
    println!("match 후 b1의 상태 {:?}", b);
    println!();
    let c = b.as_str();
    println!("{}", c);
    let d = c.to_string();
    println!("{}", d);
    println!();
    println!("재일 처음 생성한 a 바인더 상태 {}", a);

    let a2 = String::from("hello");
    let a2_1 = a2.clone();
    let a2_1_1 = a2_1.bytes();
    println!("a2_1_1 {:?}", a2_1_1);
    let a2_1_2 = a2_1.as_bytes();
    println!("a2_1_1 {:?}", a2_1_2);
    // let a = String::from("hello");
    // let z = a.clone();
    // let byte = z.bytes().nth(0);
    // println!("{:?}", byte);
    // match  {
        
    // }

}
