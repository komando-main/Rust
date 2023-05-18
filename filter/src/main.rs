// [dependencies]
// warp = "0.3.5"
// tokio = { version = "1", features = ["full"] }
use warp::Filter;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Hello {
    username: String,
}

#[tokio::main]
async fn main() {
    // let route = warp::path("hello") // http://localhost:8000/hello
    let route1 = warp::path::end()            // http://localhost:8080/
    .and(warp::fs::file("../filter/testHTML/test.html"))
        .or(warp::path("css").and(warp::path("test.css")).and(warp::fs::file("../filter/testCSS/test.css")))
        .or(warp::path("js").and(warp::path("test.js")).and(warp::fs::file("../filter/testjs/test.js")));
    
    let route2 = warp::post()
        .and(warp::path::end())
        // .and(warp::path("/"))
        // .and(warp::body::json())
        .and(warp::body::form())
        .map(|form: HashMap<String, String>| {
            let hello = Hello {
                username: form.get("username").unwrap().to_string(),//이거 왜 프린트 안찍히냐?
            };
            println!("{}", hello.username);
            format!("Hello, {}!", hello.username)
        });
        // .map(|hello: Hello| {
        //     println!("{}", hello.username);
        //     format!("Hello, {}!", hello.username)
        // });
    /*
    .and_then(|| async {
        println!("Hello, world!");
        Ok::<_, warp::Rejection>(warp::reply::with_status("Hello, world!", warp::http::StatusCode::OK))
    });
     */
    let route = route1.or(route2);
    warp::serve(route).run(([0, 0, 0, 0], 8080)).await; //// http://localhost:8000/hello
}
/*
Ok::<_, warp::Rejection>()는 Result 타입의 값을 반환한다.
Result 타입은 성공 또는 실패를 나타내는 열거형으로, Ok와 Err 두 가지 변형을 가진다.

Ok 변형은 성공적인 결과를 나타낸다. ::<_, warp::Rejection>은 Result의 타입 매개변수를 명시적으로 지정하는 것. 
여기서 _는 첫 번째 타입 매개변수가 컴파일러에 의해 추론될 수 있음을 나타내며, warp::Rejection은 두 번째 타입 매개변수로 지정.

warp::reply::with_status("Hello, world!", warp::http::StatusCode::OK) 함수는 “Hello, world!” 문자열과 HTTP 상태 코드 OK를 반환합니다.
*/