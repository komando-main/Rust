fn main() -> Result<(), String> {
    
    let result = example()?;
    
    println!("Hello, world!\n\r{}", result);

    Ok(())
}

fn example() -> Result<String, String> {
    
    let str1 = "내용 무".to_string();
    let str2 = "내용 무 투우~!".to_string();
    if true {
        // 성공적인 경우
        Ok(format!("Hello, main() -> Result<(), String> {{{str1}, {}}} \\ \\ \\\\", str2).to_string()) // String 값을 반환
    } else {
        // 실패한 경우
        Err("An error occurred".to_string()) // String 오류 반환
    }
}

// 러스트 내의 규정으로 매인의 반환은 성공했을때 main() -> Result<(), Err> {} 비어있는 유닛으로 반환해야 한다 이유는
// 성공적으로 종료되었는지 또는 오류가 발생했는지에 대한 명확한 정보를 운영 체제에 전달 해야 하기 때문이다 라고 설명하더라~!
// Rust 표준 라이브러리는 Termination이라는 트레잇을 사용하여 프로그램의 종료 상태를 처리 하기에 기본 처리방식이
// Result<(), E> 이걸로 고정되어 있다! 절대 못 바꾼다~! 규정~! 준수~!

// 간혹 규정을 무시 해야 할 상황이 오긴 한다 그럴때 규정을 무시 한 순간부터는 ******본인이 전부 모두 다 책임!!!!!!!****** 저야 한다~!!!!!!!!!!!
// 자유에는 그 만한 가치와 대가가 공존한다~!!!!! (여기서 대가란 이득이 될수도 있고 불이익이 될수도 있다~!)

/*
   Compiling main_result v0.1.0 (C:\kmj\Rust\main_result)
error[E0277]: the trait bound `String: Termination` is not satisfied
 --> src/main.rs:1:14
  |
1 | fn main() -> Result<String, String> {
  |              ^^^^^^^^^^^^^^^^^^^^^^ the trait `Termination` is not implemented for `String`, which is required by `Result<String, String>: Termination`
  |
  = note: required for `Result<String, String>` to implement `Termination`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `main_result` (bin "main_result") due to 1 previous error
*/