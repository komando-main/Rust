//Cargo.lock 파일에 적용 
// [dependencies]
// rand = "0.8.5" use rand::Rng; 이거 적용 하려면 패키지 다운 받아야 한다!
use std::io;//use std::io::*; 이런식으로 사용해도 되기는하나 이름이 겹처서 충돌 하는경우가 간혹 있다고 한다 되도록이면 use std::io::{아이템 as 키워드변경 , 아이템, ..};
use std::io::Write;//io::stdout().flush().unwrap(); 이거 사용 하기위해 적용
use std::cmp::Ordering;//주어진 값과 비교값을 확인하여 큰지 작은지 같은지를 판별할때 쓰는 옵션
use rand::Rng;//카고 패키지 적용 하면 쓸수 있다 rand::thread_rng().gen_range() 이 함수 사용을 위한 적용

fn main () {
    println!("\n숫자를 맞혀봅시다\n");

    let secret_number = rand::thread_rng().gen_range(0..=15);//1에서15 사이의 아무 숫자나 생성해주는 함수 gen_range(범위 설정은 자유로이 가능) 13..50 -> 13 서부터 49 만약 13..=50 이런 형태로 해주면 50까지 포함
    //음수 또한 무작위 생성이 가능하다
    println!("사용자가 맞혀야 할 숫자 0~15 사이의 값들 중 적어주세요! 답은 {}\n", secret_number);

    println!("종료를 원할 경우 stop 종료 exit 중에 입력하세요\n");
    
    loop {
        print!("정답이라고 생각하는 숫자를 입력 하세요! \n입력: ");
        io::stdout().flush().unwrap();//print!() 가 read_line() 로 인해 출력이 안되어 중간에 쓰래드를 잠깐 끊어 주엇다!
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("입력 값이 잘못 되었습니다!");
        
        println!();//한줄 띄우기 위해 사용 했다
        
        if guess.trim() == "stop" || guess.trim() == "종료" || guess.trim() == "exit" {//매치에서 string을 u32 로 변환 하였기때문에 글자 비교가 안되어 여기에 정의 했다
            println!("종료 합니다.\n");
            break;
        }
        
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            // Err(_) => continue,
            Err(e) => {//중괄호는 영역을 뜻하기도 하지만 순차적으로 실행하는 역활도 하다 (중광호 내에서 위에서 부터 순차적으로 실행 한다)
                                        eprintln!("입력 값이 잘못 되었습니다!\n오류원인: {e}\nplayer 가 입력한 값: {guess}\n종료를 원할 경우 stop 종료 exit 중에 입력\n");
                                        continue;
                                    },
        };

        print!("player 가 입력한 값: {}", guess);

        match guess.cmp(&secret_number) { //use std::cmp::Ordering;//주어진 값과 비교값을 확인하여 큰지 작은지 같은지를 판별할때 쓰는 옵션
            Ordering::Less => println!("\n입력한 숫자가 작습니다.\n\n종료를 원할 경우 stop 종료 exit 중에 입력\n"),
            Ordering::Greater => println!("\n입력한 숫자가 큽니다.\n\n종료를 원할 경우 stop 종료 exit 중에 입력\n"),
            Ordering::Equal => {
                                    println!("\n정답! 종료합니다.\n"); 
                                    break;
                                },
        }
    }
}