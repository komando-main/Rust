use std::ops::{Add};//제네릭 타입에 토큰 쓰는법
use std::time::{Duration};


fn main() {
    let floats = add(1.2,3.4); // 실수
    let ints = add(10, 20); // 정수
    let duration1 = add( Duration::new(5, 0), Duration::new(10, 0)); // 시간
    println!("floats : {}", floats);
    println!("ints : {}", ints);
    println!("duration : {:?}", duration1);
}

fn add<T: Add<Output = T>>(i:T, j:T) -> T {//트레이드 제약(+, - , *, /, %, 등등) < T:Add<Output = T> > 오... 이런 방법이...
    i+j
}