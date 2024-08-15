#![allow(unused_mut)]
fn main() {
    let mut a = Some(String::from("asd"));
    println!("{}", a.unwrap());

    // let out_value = match opt(4) {
    //                         Ok(a)=> a,
    //                         Err(a)=>{
    //                             println!("{a}");
    //                             0
    //                         }
    //                     };

    let out_value = opt(4).unwrap_or_else(|err| {
                                                            println!("Error: {}", err);  // 오류 메시지를 출력
                                                            -1  // 대체 값 반환
                                                        });
    // let out_value = opt(4).unwrap_or(0);
    println!("{out_value}");
    let out_value = opt(4).unwrap_err();
    println!("{out_value}");
    // match out_value {
    //     Ok(a)=>println!("{a}"),
    //     Err(a)=>println!("{a}")
    // }
}

fn opt(a:i32)->Result<i32, String>{
    
    // if a == 3 {
    //     Ok(a)
    // } else {
    //     Err("no data".to_string())
    // }

    match a {
        3 => {Ok(a)},
        _ => {Err("no data".to_string())},
    }
        
}