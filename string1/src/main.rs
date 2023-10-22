#![allow(unused_assignments)]

fn str_part_equals(a:&str, b:usize, c:usize, d:&str) -> usize/*bool*/ {//bool 타입 반환도 가능하나 1과 0으로 반환 하라 하여 usize 반환 했다!
    let mut f:usize = 0;
    // println!("{}", &a[b..c]);
    if &a[b..c] == d {
        f=1
        //return true;
    }
    f
}

fn ends_with(a: &str, b: &str) -> bool {
  a.ends_with(b)//러스트 문서에 찾으면 나온다!
}
/*
https://doc.rust-lang.org/std/primitive.str.html#method.ends_with
let bananas = "bananas";
println!("{}", bananas.ends_with("anas")); true
println!("{}", bananas.ends_with("nana")); false
*/
fn ends_with1(a: &str, b: &str) -> bool {
  a.contains(b)
  // false
}


fn main() {
    // str_part_equals("abcd", 0, 2, "ab");
    // let a = "abcd";
    // let a1 = &a[0..2];
    // println!("{}", a1);
    println!("str_part_equals(\"abcd\", 0, 2, \"ab\") : {}", str_part_equals("abcd", 0, 2, "ab"));
  // 출력 => str_equals("abcd", 0, 2, "ab") : 1

    println!("str_part_equals(\"abcd\", 1, 2, \"b\") : {}", str_part_equals("abcd", 1, 2, "b"));
  // 출력 => str_equals("abcd", 1, 2, "b") : 1

    println!("str_part_equals(\"abcd\", 2, 2, \"\") : {}", str_part_equals("abcd", 2, 2, ""));
  // 출력 => str_equals("abcd", 2, 2, "") : 1

    println!("str_part_equals(\"abcd\", 2, 4, \"cb\") : {}", str_part_equals("abcd", 2, 4, "cb"));
  // 출력 => str_equals("abcd", 2, 4, "cb") : 0

    println!("str_part_equals(\"abcd\", 2, 4, \"cd\") : {}", str_part_equals("abcd", 2, 4, "cd"));
  // 출력 => str_equals("abcd", 2, 4, "cd") : 1
    let mut rs:bool = false;
    rs = ends_with("abc", "bc");
    println!("rs : {}", rs); // 출력 rs : true
    
    rs = ends_with("kbsa", "kbs");
    println!("rs : {}", rs); // 출력 rs : false

    rs = ends_with("kbs", "bs");
    println!("rs : {}", rs); // 출력 rs : true

    rs = ends_with("mbc", "mc");
    println!("rs : {}", rs); // 출력 rs : false

    println!("--------------------------------------------------------------");

    rs = ends_with1("abc", "bc");
    println!("rs : {}", rs); // 출력 rs : true
    
    rs = ends_with1("kbs", "kb");
    println!("rs : {}", rs); // 출력 rs : false
    
    rs = ends_with1("abcdefghij", "def");
    println!("rs : {}", rs); // 출력 rs : true 어떻게 해야 하니?

    rs = ends_with1("mbc", "mc");
    println!("rs : {}", rs); // 출력 rs : false


}