fn main() {
    let n:f64 = -419.42;
    let n_bits:u64 = n.to_bits();
    let sign_bits = n_bits >> 63; 
    let sign_bits = sign_bits << 63;         // & 8000000000000000
    let exponent_bits = n_bits << 1;
    let exponent_bits = exponent_bits >> 53; 
    let exponent_bits = exponent_bits << 52; // & 7ff0000000000000
    let mantissa_bits = n_bits << 12;
    let mantissa_bits = mantissa_bits >> 12; // & 000fffffffffffff
    let combine_bits = sign_bits + exponent_bits + mantissa_bits;
    let combine_f64 = f64::from_bits(combine_bits);//비트를 그대로 f64에 표현한다 as f64는 안됨

    println!("부호 {:064b}", sign_bits);
    println!("지수 {:064b}", exponent_bits);
    println!("가수 {:064b}", mantissa_bits);
    println!("\n원본 {:064b}", n_bits);
    println!("결합 {:064b}", combine_bits);
    println!("원본 {}, 결합 {}", n, combine_f64);
    
}