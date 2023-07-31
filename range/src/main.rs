fn main() {

    let range:Vec<_> = (1..=3).collect();
    println!("{:?}", range);
    
    let range2:Vec<_> = (1..4).collect();//4까지 포함하려고 하면 =4 라고 표현 해줘야 한다
    println!("{:?}", range2);
    
    for num in 1..4 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }

}
// 범위 표현은 끝에 = 기호가 있는냐 없는냐 에 따라 표현 되는 방식이 다르기에 주위해야 한다
// 예를 들어 0..10 은 컴퓨터가 이해 하기를 0 부터 9 까지 표현 하라고 인식할수 있다
// 하지만 0..=10 이면 0부터 10 까지 표현 함으로 = 기호가 있고 없고의 차이가 있다!