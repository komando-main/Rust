use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10;//<스택영역>에 정수를 생성한다
    let b = Box::new(20);//<힙영역>에 정수를 생성한다 박스된(boxed) 정수라고도 한다
    let c = Rc::new(Box::new(30));//참조 카운터 안에 <힙영역>박스된 정수를 담는다
    let d = Arc::new(Mutex::new(40));//원자적(atomic) 참조 카운터에 담긴 정수이며, 상호 배재(mutual exclusion)잠금 방식으로 보호받는다.
    println!("a={:?}, b={:?}, c={:?}, d={:?}", a, b, c, d);
}
//가비지 컬랙터가 없어도 참조를 사용하기 쉬워지며 데글링 포인트 함점에 걸리지 않을수있다
//단 시스템 자원을 많이 소비 할 수 있다는 단점도 있다