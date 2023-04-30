/*

Cargo.toml 파일에서
[dependencies]
regex = "1.5"   <---이거 정의 해줘야

use regex::Regex; <---이걸 쓸수 있다
*/
use regex::Regex;// regex 크리에이트의 Regex 타입을 지역 범위로 가져온다
fn main() {
    
    let re = Regex::new("picture").unwrap();//.unwrap()은 Result 값을 풀때 오류가 발생하면 강제 종료한다 
    
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with book. What do we seek through millions of pages?";
    


    for line in quote.lines(){//.lines()가 반복자를 반환
        let contains_substring = re.find(line);
        match contains_substring { //예제 2.23의 contains() 메서드 블록으로 대치하여 발생 가능한 모든 경우으ㅔ 대처한다
            Some(_) => println!("{}", line),//Some(T)는 Option 타입의 값 중 긍정적인 경우로 re.find() 가 성공했다는 의미다 결과가 있는 모든 경우에 해당한다
            None => (),//None은 Option 타입의 값 중 부정적인 경우다 ()는 널 표시자로 볼 수 있다 하지만 NULL은 표시도 사용도 못한다!
        }
    }
}