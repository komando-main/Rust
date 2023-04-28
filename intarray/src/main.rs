fn main() {
    let needle = 0o204;
    println!("8진수를 10진수로 변환 = {}", &needle);
    println!("8진수 화면 출력 = {:o}", &needle);
    let haystack=[1,2,5,15,52,132,203,877,4140,21147];

    for item in &haystack{
        if *item == needle{//*item 은 for문의 item 에서 역참조 된것이다!!
            println!("----------------------------for 안의 if 문---------------------------------");
            println!("needle의 8진수가 10진수로 변경되어 적용 = {}", item);
            println!("----------------------------for 안의 if 문 끝---------------------------------");
        }
    }
}