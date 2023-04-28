use std::collections::HashMap;
fn main() {
    println!("--------------------------------------------------------main() 시작-----------------------------------------------------------");
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    let list1 = ["asd".to_string(),"qwe".to_string(),"zxc".to_string()];//String::from("블라블라") 이걸로 안하면 죽어도 안넘어간다.. 대처한 꼼수 .to_string() ㅎㅎㅎㅎ

    //println!("lll(list: &[String])->Vec<String> 값 넘겼다!!!! {:?}", lll(&list1)); //변수 만들기 귀찬아서 그냥 함수 호출!! 프린터에 ㅎㅎㅎ

    //println!("lll(list: &[String])->Vec<String> 값 넘겼다!!!! {}", lll(&list1)[0]);
    println!("lll(list: &[String])->Vec<String> 값 넘겼다!!!! lll(&list1)[1] = {}", lll(&list1)[1]);

    let mut asd:HashMap<String, String> = HashMap::new();//아무것도 없는 비어있는 상자와 같은 상태

    asd.insert(String::from("asd"), String::from("vvv"));//insert 할때 key, value 는 같이 들어가야 한다
    asd.insert(String::from("a1"), String::from("vvvvv"));
    asd.insert(String::from("s2"), String::from("vvvvvvv"));
    
    println!("key kmj의 value 값은 {}", asd1(asd).get("kmj").unwrap()); //변수 만들기 귀찬아서 그냥 함수 호출!! 프린터에 ㅎㅎㅎ

    let c = qwe(&haystack);
    
    
    println!("qwe() 함수 {:?}", c);

    for item in &haystack {
        let result = match item {//match 표현식은 바인드될 값을 반환한다.
            42| 132 =>"hit",
            _ => "miss",
        };

        if result == "hit" {
            println!("{}: {}", item, result);
        }
    }
    //이거 전에 찾아보긴 했는대 역참조 햇갈려서 안썻는대 조금이나마 이해 했다...
    let string_thing = String::from("asd");
    match &*string_thing {
        "a" => println!("string_thing 0 : asd"),
        "asd" => println!("string_thing 1 : asd"),
        "c" => println!("string_thing 2 : asd"),
        _ => println!("something else!"),
    }    

    println!("--------------------------------------------------------main() 끝-----------------------------------------------------------");
}

fn qwe(list:&[i32]) -> Vec<String> {
    
    let mut b: Vec<String> = Vec::new();

    for item in list{
        let a = match item {
            42 | 132 =>"hit",
            _ => "miss",
        };
        b.push(a.to_string());//for 문 안에서 변경 되는대... 왜 책에서는 안된다고 할까 구조가 똑같은대... 2018년도 책이라 그런가...;; <---잘못 이해함
    }                         //<<<<<<음 재대로 안봄 잘됨 값이 변경되는 것이 아닌 비교 하여 다른 영역의 비교문의 값을 출력 해주는것 ㅎㅎㅎㅎㅎㅎㅎㅎㅎㅎㅎ>>>>>

    b
}
fn asd1(mut map : HashMap<String, String>) -> HashMap<String, String>{//인자 에 직접 생성 할수 있다!!! 중요!!!! mut map 라는 바인드 명 생성 속성은 해쉬맵 타입
    println!("--------------------------------asd1() 함수 시작--------------------");
    map.insert(String::from("kmj"), String::from("123"));
    
    for (key, value) in &map{
        println!("--------------------------------asd1() 안의 for문 함수 시작--------------------");
        println!("{}:{}", key, value);
        println!("--------------------------------asd1() 안의 for문 함수 끝--------------------");
    }
    println!("--------------------------------asd1() 함수 끝--------------------");
    map
}
fn lll(list: &[String])->Vec<String>{
    println!("------------------------------------------------fn lll() 영역 시작------------------------------------------------");
    let list1 = list.to_vec();//for 문에서 쓰기위한 백터 변경
    let mut list2:Vec<String>=Vec::new();//담아서 보낼 상자 생성
    for a in &list1{
        println!("------------------------------------------------fn lll() 영역 안의 for 시작------------------------------------------------");
        let f = match &*a.to_string() {//이개 역참조 라는거구나... 한참을 해매었다... 문자열은 역시 빡새다.... (*) 역참조, (&) 참조, (&*) 참조한곳에 역참조라는건가 ㅎㅎ &[*원소] 요래 이해 하면 될라나?
            "a" => "aa",
            "asd" => "ss_asd",
            "zxc" => "dd_zxc",
            _ => "",
        };
        if f.trim().len() > 0 {
            println!("for 문 안의 if 문 프린트 {}", f);
            list2.push(f.to_string());
            
        }
        println!("------------------------------------------------fn lll() 영역 for 끝------------------------------------------------");
    }
    println!("------------------------------------------------fn lll() 영역 끝------------------------------------------------");
    list2
}