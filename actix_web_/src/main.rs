use actix_web::{App, HttpServer, Result, HttpResponse, get, post, web};//서버 만들때
use actix_files::NamedFile;//html에 CSS JS 적용 하려 할때
use serde::{Deserialize, Serialize};//포스트 방식의 데이터를 받을때
use askama::Template;//html 파일에서 for if 등을 사용 하고 싶을때 정의
#[allow(unused_imports)]
use serde_json::{json, to_string, Value};//json 방식 사용

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]//post 방식으로 받을때 사용
struct MyForm {//묶음 데이터 이기때문에 양식이 틀리면 오류난다
    username: String,
}
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]//post, json 사용시 설정
struct FormData {
    text: String,
}

#[allow(dead_code)]
#[derive(Template)]//template을 이용하여 html 파일 사용 설정
#[template(path = "test.html")]//html 에서 if for 등 자동으로 생성 할때 쓰는법 폴더 확인 잘 하자 templates 뒤에 S 붙어야 한다
struct List {
    list: Vec<String>,
}

#[get("/")]//요청 받았을때
async fn index() -> HttpResponse {//요청 보내때 쓰는 리턴 ***** HttpResponse *****
    let a = List {//요청 보낼 함수에 직접 데이터 입력 해야 한다
        list: vec!["사과".to_owned(), "바나나".to_owned(), "레몬".to_owned(), "체리".to_owned()],
    };
    let rendered_template = a.render().unwrap();//데이터 읽어서 변환하여 보낸다
    HttpResponse::Ok().body(rendered_template)//보낼때 쓰는 함수
}

#[get("/test.css")]
async fn css() -> Result<NamedFile> {//파일 보낼때 쓰는 방법
    Ok(NamedFile::open("CSS/test.css")?)//실재 파일 위치 하고 파일 이름과 확장자 지정 
}

#[get("/test.js")]
async fn js() -> Result<NamedFile> {
    Ok(NamedFile::open("js/test.js")?)
}

#[post("/submit")]//포스트 방식으로 받을때 사용
async fn submit(form: web::Form<MyForm>) -> HttpResponse {//포스트 방식으로 받을때 인자 값에 web::Form<MyForm> 이런식으로 해줘야 한다
    println!("Received form data: {:?}", form);
    println!("Received form data: {}", form.username);
    HttpResponse::Ok().body("Good ^_^/")//다시 웹 패이지로 보넬때
}

#[post("/json")]
async fn api_v1(form: web::Json<FormData>) -> HttpResponse {
    println!("Received form data: {:?}", form);
    println!("Received form data: {}", form.text);
    let body = to_string(&form).unwrap();
    println!("Received form data: {}", body);
    HttpResponse::Ok().content_type("application/json").body(body.to_string())//content_type()보낼 데이터 유형 설정
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {//매인 자채가 in out를 반환 한다
    HttpServer::new(|| App::new()
        .service(
            (//위에 정의한 함수 이름이 인자 값으로 들어간가 함수 만들었으면 여기에 적어라 재발!!
                index, 
                css, 
                js, 
                submit, 
                api_v1
            )
        )
    )
    .bind("127.0.0.1:8888")?//서버 IP 지정 웹상에 노출하는 방법 내컴퓨터 공인 IP 적어주면 잘된다~!
    .run()//실행
    .await//비동기 함수 쓸때 사용 (실행 또는 중지) async 비동기 키워드
}