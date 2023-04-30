fn main() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with book. What do we seek through millions of pages?";
    

//for 문을 이용한 문자열 찾기
    for line in quote.lines(){//.lines()가 1라인 씩 반환 한다
        if line.contains(search_term) {//contains() 인자에 찿고자 하는 단어의 라인을 찾아준다.
            
            println!("{}", line);
        }
    }
}