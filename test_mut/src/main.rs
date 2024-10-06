fn main() {
    let mut a = 10;

    // *&mut a += 1;
    
    // let b = &a;
    
    // println!("{} {}", b ,a);
    let mut r3 = ();
    println!("{:?}", r3);
    if r3 == () {
        println!("참");
    }
    *&mut a += 2;
    {
        // let r1 = (*&mut a += 3).clone()*//*  + 1*/; // 불변 참조
        let r1 = *&mut a += 3;
        // if &mut a == &r1 {
        //     println!("if &mut a == &r1 참");
        // } else {
        //     println!("if &mut a == &r1 거짓");
        // }
        println!("{:p} {:p} {:?}", &r1, &mut a, r1);
        // println!("{:p} {:p} {:p}", &r1, &mut a, r1);
        r3 = r1;
    }

    let r2 = *&mut a += 1/*.clone()*/;
    println!("{:?}", a);
    println!("{:?}", r2);
    println!("{:?}", r3);
    
    let p1 = 1;
    let p2 = &p1;

    println!("{:p} {:p}", &p1, p2);
    println!("{:p} {}", &p1, p2);
    // println!("{b}");

    
    // let r1 = &a;  // 불변 참조 r1
    // let r2 = &mut a;  // 여기서 가변 참조 r2를 시도하면 오류 발생
    
    // println!("{:?}", r1);  // 불변 참조 r1이 아직 사용 중인 상태
    // println!("{:?}", r2);  // 오류 발생

    // let asd = None;
    let asd;
    let asd1 = 1;
    asd = &asd1;
    println!("{:?}", asd);
    
    // let mut asdf = Some(None);
    let mut asdf = Some(None);
    let asdf1 = 1;
    // asdf = Some(Some(asdf1));
    asdf = Some(Some(&asdf1));
    // asdf.unwrap() += asdf1;
    println!("{:?}, {:?}, {:?}", asdf, asdf.unwrap(), asdf.unwrap().unwrap());
    println!("{:p}, {:p}, {:p}", &asdf, &asdf.unwrap(), &asdf.unwrap().unwrap());

    // println!("{:?}", asdf.unwrap().unwrap());

    // let qwe:Option<_> = None::<T>;
    let qwe = Some(1);
    println!("{:?}", qwe);

    let zxc;
    let qaz = (44,55,66);
    zxc = (&qaz.1, &qaz.2);

    let mut zxc1;
    let qaz1 = (11,22,33);
    zxc1 = &qaz1.2;
    println!("{:?}, {:?}", zxc, zxc1);
    println!("{:p}, {:p}", &qaz.1, &qaz.2);
    let (qaz, qaz1) = zxc;

    println!("{:p}, {:p}", qaz, qaz1);

    let o:&u8 = &1;
    println!("{:p}", &o);
    println!("{:p} , {:p}", &*o, &1);//
    println!("{:p}", &1);
    println!("{:p}", &1);

    println!("{}", o);
    println!("{}, {:p}", &1, &1);

    let binary_string = format!("{:08b}", o); // 이진수로 변환
    let formatted: String = binary_string
        .as_bytes()               // 바이트로 변환
        .rchunks(4)               // 4개씩 나눔 (역순으로 나눔)
        .rev()                    // 원래 순서로 돌림
        .map(|ch| std::str::from_utf8(ch).unwrap())  // 다시 문자열로 변환
        .collect::<Vec<&str>>()    // 벡터로 수집
        .join(" ");               // 공백으로 결합

    println!("{}", formatted); // 출력: "0000 0001"

    let binary_string = format!("{:08b}", 1);  // 이진수로 변환 (1을 8자리로)
    let formatted: String = binary_string
        .chars()                               // 문자를 하나씩 나눔
        .collect::<Vec<char>>()                // Vec<char>로 변환
        .chunks(4)                             // 4개씩 나눔
        .map(|ch| ch.iter().collect::<String>()) // 각 ch 다시 문자열로 변환
        .collect::<Vec<String>>()              // Vec<String>으로 수집
        .join("_");                            // 각 문자를 '_'로 연결
    println!("{}", formatted);

}
