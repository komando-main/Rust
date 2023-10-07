use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("../aaa.txt").unwrap();//File 객체를 만들 때는 경로 인자가 필요하며 파일이 존재하지 않는 경우 오류가 발생한다 이 프로그램의 경우 aaa.txt가 존재하지 않는다면 강제 종료된다.
    let reader = BufReader::new(f);


    for line_ in reader.lines(){//여기에서 미묘한 변화가 일어났다. BufReader::lines()는 각 줄에서 맨 뒤 개행 문자를 제거한다
        let line = line_.unwrap();//Result를 푼다. 하지만 오류 발생시 프로그램이 강제 종료되는 위험을 감수한다
        println!("{} ({} bytes long)", line, line.len());//문자의 길이 bytes
    }

}