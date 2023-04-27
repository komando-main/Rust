fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));// 자기자신을 집어 널수 있다!! 제귀함수!!

    println!("제귀호출_함수 (a+b) + (c+d) = {} ", e);

    let mut f:u8 = 3;
    f+=f;
    println!("제귀호출_바인드(변수) f+=f {}", f)

}
fn add(i:i32, j:i32)->i32{
    i+j
}