use std::io;
use std::io::Write;
use std::any::type_name;//이걸 써줘야 type_name::<T>() 이걸쓸 수 있다

fn type_of<T>(_: &T) -> &str {//제네릭으로 바로 보낼때 인자에 바인드가 1개인 경우 _ (이름을 지정하지 않음) 으로 보낸다 도있지만 타입을 알아서 추론 하라 라는 뜻도 된다
    type_name::<T>()//반환 타입 &str
}
fn main() {
    let mut a = String::new();
    
    print!("숫자를 입력하세요!\n입력 : ");
    io::stdout().flush().unwrap();
    
    let _ = io::stdin().read_line(&mut a);

    let a1:u8 = loop {
        match a.trim().parse() {
            Ok(num) => break num,
            Err(e) => {
                println!("잘못 입력 했습니다!");
                println!("원인 : {}", e);
                println!("다시 입력 하세요!");
                io::stdout().flush().unwrap();
                a.clear();
                io::stdin().read_line(&mut a).expect("입력을 읽지 못했습니다.");
            },
        }
    }; 

    
/*//와일문 활용
    let mut a = String::new();

    print!("숫자를 입력하세요!\n입력 : ");
    io::stdout().flush().unwrap();

    let mut a1: Option<u8> = None;

    while a1.is_none() {
        io::stdin().read_line(&mut a).expect("입력을 읽지 못했습니다.");
        a1 = match a.trim().parse() {
            Ok(num) => Some(num),
            Err(e) => {
                println!("오류");
                println!("원인 : {}", e);
                println!("다시 입력 하세요!");
                a.clear();
                None
            },
        }
    }

    let a1 = a1.unwrap();
*/   
    println!("타입 {}, 값 {}", type_of(&a1), a1);
}