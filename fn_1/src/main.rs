fn main() {
    let al = 10;
    println!("1번 {}", ex1(al));
    let al = 3;
    println!("2번 {}", ex1(al));

    let ad = 8;
    println!("3번 {}", ex2(ad));


}
fn ex1(num:i32)->i32{
    if num>5{
        num
    }else{
        0
    }

}
fn ex2(num:i32)->i32{
    match num{
        1 => 1+9,
        2 => 2+9,
        _ => num,
    }
}
