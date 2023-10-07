use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("../aaa.txt").unwrap();//File 객체를 만들 때는 경로 인자가 필요하며 파일이 존제하지 않는 경우 오류가 발생한다 이 프로그램의 경우 readme.md가 존재하지 않는다면 강제 종료된다.
                                           //기본 경로는 프로잭트 폴더의 /src (main.rs 파일과 같은 폴더내에 있어야 한다)부터이다 ex)/src 폴더 내부에 찾고자 하는 파일이 있어야 한다 ./현재 폴더 ../상위 폴더
    let mut reader = BufReader::new(f);

    let mut line = String::new();//하나의 String 객체를 프로그램 수명 내내 재활용 한다

    loop {
        let len = reader.read_line(&mut line).unwrap();//디스크 읽기가 실패할 수 있으니 이를 명시적으로 처리할 필요가 있다 이경우에는 실패할 때 프로그램을 강제 종료한다.

        if len == 0 {
            break
        }
    }

    println!("{} ({} bytes long)", line, line.len()); 
    
    line.truncate(0);//해당 String 객체의 길이를 0으로 줄인다 다음번 반복에서 기존에 있는 값이 재활용되는 것을 막기 위해서이다.
}