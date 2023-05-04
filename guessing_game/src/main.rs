use std::io;//스텐다드 인풋 아웃풋을 사용하기 위한 정의

fn main() {

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();//비어있는 스트링 가변 바인드를 만든다

    io::stdin().read_line(&mut guess).expect("Failed to read line");// 키보드로 부터 문자를 입력받아 guess 바인드에 저장하기

    println!("You guessd: {}", guess);//저장된 바인드값 출력

}
