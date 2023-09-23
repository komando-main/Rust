fn main() {
    let n:f64 = -419.42;
    let n_bits:u64 = n.to_bits();            // 비트 로 저장 할때 사용 하는 함수 to_bits() 원본 비트 1100000001111010001101101011100001010001111010111000010100011111
    
    let sign_bits = n_bits >> 63; 
    let sign_bits = sign_bits << 63;         // & 8000000000000000 16진수 &연산자 비교로도 가능하다
    
    let exponent_bits = n_bits << 1;
    let exponent_bits = exponent_bits >> 53; 
    let exponent_bits = exponent_bits << 52; // & 7ff0000000000000 16진수 &연산자 비교로도 가능하다
    
    let mantissa_bits = n_bits << 12;
    let mantissa_bits = mantissa_bits >> 12; // & 000fffffffffffff 16진수 &연산자 비교로도 가능하다
    
    let combine_bits = sign_bits + exponent_bits + mantissa_bits; // 비트를 전부 더한다
    let combine_f64 = f64::from_bits(combine_bits);// f64::from_bits() 비트를 그대로 f64에 표현한다 as f64는 안됨

    println!("원본 비트 {:064b}", n_bits);
    println!("부호 분리 {:064b}\n", sign_bits);
    println!("원본 비트 {:064b}", n_bits);
    println!("지수 분리 {:064b}\n", exponent_bits);
    println!("원본 비트 {:064b}", n_bits);
    println!("가수 분리 {:064b}\n", mantissa_bits);
    println!("\n원본 {:064b}", n_bits);
    println!("결합 {:064b}\n", combine_bits);
    println!("원본 {}, 결합 {}", n, combine_f64);

    if n == combine_f64 {
        println!("원본과 같은지 if 로 태스트 했을때 같다!");
    } else {
        println!("원본과 같은지 if 로 태스트 했을때 틀리다 뭐가 틀린거지?");
    }
    
}