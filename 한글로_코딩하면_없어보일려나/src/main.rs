fn main() {

    let 벡터 = vec!["이거 될까?"];
    let 벡터2 = vec!["이것도?".to_owned()];
    
    println!("{}",값(&벡터));

    println!("{}",값2(벡터2));
}
fn 값<'a>(벡터:&'a Vec<&'a str>)->&'a str{
    &벡터[0]
    
}

fn 값2(벡터:Vec<String>)->String{
    벡터[0].clone()
}