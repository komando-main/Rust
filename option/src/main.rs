#![allow(unused_mut)]
fn main() -> Result<(), String>{
    let mut a = Some(String::from("asd"));
    println!("{}", a.unwrap());

    // let out_value = match opt(4) {
    //                         Ok(a)=> a,
    //                         Err(a)=>{
    //                             println!("{a}");
    //                             0
    //                         }
    //                     };

    // let out_value = opt(Some(4)).unwrap_or_else(|err| {
    //                                                         println!("Error: {}", err);  // 오류 메시지를 출력
    //                                                         -1  // 대체 값 반환
    //                                                     });
    // let out_value = opt(4).unwrap_or(0);

    let out_value = opt(Some(4))?;

    println!("{out_value}");
    // let out_value = opt(Some(4)).unwrap_or_else(|err| {
    //     println!("Error: {}", err);  // 오류 메시지를 출력
    //     -1  // 대체 값 반환
    // });
    let out_value = opt(None).unwrap_or_default();
    println!("{out_value}");
    // match out_value {
    //     Ok(a)=>println!("{a}"),
    //     Err(a)=>println!("{a}")
    // }

    // let out_value = opt(None)?;
    // println!("{out_value}");

    Ok(())
}

fn opt(a:Option<i32>)->Result<i32, String>{
    
    // if a != None {
    //     Ok(a.unwrap())
    // } else {
    //     Err("no data".to_string())
    // }

    // if !a.is_none() {
    //     Ok(a.unwrap())
    // } else {
    //     Err("no data".to_string())
    // }

    // match a {
    //     Some(a) => Ok(a),
    //     _ => Err("no data".to_string()),
    // }
    
    // let num = a.ok_or("no data".to_string())?;
    // Ok(num)

    Ok(a.ok_or("no data".to_string())?)
}