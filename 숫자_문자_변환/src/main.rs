fn main() {

    let a = '9';//char 타입 '' 0~9 까지 1글짜만 된다
    let b: u8 = 3;
    let result = a.to_digit(10).unwrap() as u8 + b;//숫자 변환 10진수로 해석 as 로 타입 변환가능능
    
    let c = "222";//string 타입 "" 변환 타입에따른 재약만 있다 
    let result1: u8 = c.parse::<u8>().unwrap() + b;
    
    
    let a1 = 1234;
    let a2 = a1.to_string();//string 타입으로 변환
    let a3 = 1;
    let a4 = std::char::from_digit(a3, 10).unwrap();//char 타입 숫자 변환 10진수로 해석
    
    println!("{:?}", a4);
    println!("{:?}", a2);
    println!("{:?}", result);
    println!("{:?}", result1);
}