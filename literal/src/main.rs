fn main() {
    let twenty = 20; //기본적으로 컴퓨터 사양에 따라 32bit 또는 64bit로 인식한다 컴파일될때 코드에따라 자동으로 타입을 변경하기도 한다
    let twenty_one: i32 = 21;//i32로 직접 정해준 방법
    let twenty_two=22i32;//값 뒤에 정의한 방식 접미사 라고도 한다 왜 디아블로2 item 보면 접두사 접미사 뭐 붙었냐에 따라 옵션이 재각각 이잔아...
    let addition = twenty+twenty_one+twenty_two;// 자동으로 타입을 변경 하여 대입한다 타입이 1개라도 틀리면 컴파일이 안된다!
    println!("{}+{}+{}={}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64=1_000_000;// ( _ ) 언더바로 구분지을 수 있어서 확인하기 쉽다!!
    println!("{}", one_million);

    let forty_twos = [42.0, 42f32, 42.0_f32];//실수도 접미사로 정의가 가능하다
    println!("{:02}", forty_twos[0]);

    let a = -0x_ffff_i64;//접두사 접미사 전부 값에 쓸수있다.
    println!("-0x_ffff_i64 = {}", a);

}