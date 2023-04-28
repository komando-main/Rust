fn main() {
    let a = 13;
    let b = 50;
    let c = add_with_life_time(&a, &b);// & 참조
    println!(" *i + *j = {}", c);
}

fn add_with_life_time<'a, 'b>(i:&'a i32, j:&'b i32) -> i32 {//<'a, 'b> 참조자의 수명 에너테이션
    *i + *j   // * 곱샘도 되지만 여기서는 역참조 라는 뜻이다!!
}