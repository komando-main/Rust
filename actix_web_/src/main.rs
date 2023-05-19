use actix_web::{App, HttpServer, Result, HttpResponse, get, post, web};
use actix_files::NamedFile;
use serde::Deserialize;
use askama::Template;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]//post 방식으로 받을때 사용
struct MyForm {
    username: String,
}

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "test.html")]//html 에서 if for 등 자동으로 생성 할때 쓰는법 폴더 확인 잘 하자 templates 위에 S 붙어야 한다
struct List {
    list: Vec<String>,
}

#[get("/")]//요청 받았을때
async fn index() -> HttpResponse {//요청 보내 쓰는 리턴 ***** HttpResponse *****
    let a = List {//요청 보낼 함수에 직접 데이터 입력 해야 한다
        list: vec!["사과".to_string(), "바나나".to_string(), "레몬".to_string(), "체리".to_string()],
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

#[post("/submit")]
async fn submit(form: web::Form<MyForm>) -> HttpResponse {//포스트 방식으로 받을때 web::Form<MyForm> 이런식으로 해줘야 한다
    println!("Received form data: {:?}", form);
    HttpResponse::Ok().body("Good ^_^/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service((index, css, js, submit)))//위에 정의한 함수 이름이 인자 값으로 들어간가
        .bind("127.0.0.1:8888")?//서버 번호 지정 웹상에 노출하는 방법 내컴퓨터 공인 IP 적어주면 잘된다~!
        .run()//실행
        .await
}