fn main() {
    let a = 13;
    let b = 50;
    let c = add_with_life_time(&a, &b);// & 참조
    let d = add_with_life_time1(&a, &b);
    println!(" *i + *j = {}", c);
    println!(" *n + *h = {}", d);
}

fn add_with_life_time(i:&i32, j:&i32) -> i32 {
    *i + *j   // * 곱샘도 되지만 여기서는 역참조 라는 뜻이다!!
}

fn add_with_life_time1<'a, 'b>(n:&'a i32, h:&'b i32) -> i32 {//<'a, 'b> 참조자의 수명 에너테이션
    *n + *h   // * 곱샘도 되지만 여기서는 역참조 라는 뜻이다!!
}