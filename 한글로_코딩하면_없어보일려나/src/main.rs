fn main() {

    let 벡터 = vec!["이거 될까?"];
    let str1 = "이건 되냐?";
    let 벡터2 = vec!["이것도?".to_owned()];
    let str2 = String::from("예도 되니?");
    
    println!("{}",값(&벡터));
    println!("{}",value1(str1));
    println!("{}",값2(벡터2));
    println!("{}", value2(str2));
}
fn 값<'a>(벡터:&'a Vec<&'a str>) -> &'a str {
    &벡터[0]   
}

fn value1(v:&str) -> &str {
    &v
}

fn 값2(벡터:Vec<String>)-> String {
    벡터[0].clone()
}

fn value2(s:String) -> String {
    s
}