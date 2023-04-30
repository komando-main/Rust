use regex::Regex;
use clap::{App,Arg}; //clap::App, clap::Arg 객체를 지역 범위로 가져온다
//cargo run -- picture 으로 실행 (-- 뒤에 오는 모든 인자는 실행 가능한 바이너리로 전달된다)
fn main() {
    let args = App::new("grep-lite") //명령인자 분석기를 점진적으로 구성한다. Arg를 통해 각 인자를 가저온다. 우리의 경우에는 하나만 필요하다
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true)
            )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap(); //pattern 인자를 축출한다.
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with book. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}