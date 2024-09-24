fn main() {
    let a = 13;
    let b = 50;
    let c = add_with_life_time(&a, &b);// & 참조
    let d = add_with_life_time1(&a, &b);

    let arr = [1,2,3];


    println!(" *i + *j = {}", c);
    println!(" n + h = {}", d);
    println!(" a = {}", a);
    println!(" b = {}", b);
    println!(" arr = {:?}", arr);
    println!("arr[0] + arr[1] + arr[2] = {}", add_arr(&arr));

}

fn add_with_life_time(i:&i32, j:&i32) -> i32 {
    *i + *j   // * 곱샘도 되지만 여기서는 역참조 라는 뜻이다!! 참고로 역참조되어진 2 값은 새로운 바인드에 저장된다 let _ = *i + *j 이상태라고 보면 된다!
}

fn add_with_life_time1<'a, 'b>(n:&'a i32, h:&'b i32) -> i32 {//<'a, 'b> 참조자의 수명 에너테이션
    n + h   // 숫자와 같은 간단한 값은 역참조를 안해도 된다!
}

fn add_arr(arr:&[i32]) -> i32 {
    arr[0] + arr[1] + arr[2]
}