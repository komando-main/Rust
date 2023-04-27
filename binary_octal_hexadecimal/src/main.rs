fn main() {
    let three = 0b11;// 0b 접두사 이진수
    let thirty = 0o36;// 0o 접두사 팔진수
    let three_hundred = 0x12c;// 0x 접두사 십육진수

    println!("base 10: {} {} {}", three, thirty, three_hundred);//10진수로 표현 하라 {}
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);//2진수로 표현 하라 {:b} b=바이너리 약자
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);//8진수로 표현 하라 {:o}
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);//16진수로 표현 하라 {:x}
}