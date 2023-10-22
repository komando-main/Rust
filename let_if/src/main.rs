#![allow(dead_code)]//코드 경고 무시
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {

    let maybe_user = Some("Jerry");
    if let Some(user) = maybe_user {//1가지만 구분 할 경우 match 보단 if let 이 더 빠르기 때문에 1가지의 분기만 필요시 사용하면 유용 하다
        println!("user={:?}", user);
    } else {
        println!("no user");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red!");
    }else{
        println!("it's not red");
    }

    

    let col = "Red";
    let match1 = match col {
        "Red" => {
            println!("red");
            println!("{:?}", col);
            Color::Red
        },
        _ => {
            println!("no Color");
            Color::Green  // 기본값으로 Green을 반환
        },
    };

    println!("{:?}", match1);

}
