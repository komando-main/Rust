fn main() {
    let three = 0b11;// 0b 접두사 2진수 0~1 01
    let thirty = 0o36;// 0o 접두사 8진수 0~7 01234567
    let three_hundred = 0x12c;// 0x 접두사 16진수 0~f 0123456789ABCDEF

    println!("base 10: {} {} {}", three, thirty, three_hundred);//10진수로 표현 하라 {}
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);//2진수로 표현 하라 {:b} b = binary
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);//8진수로 표현 하라 {:o} o = octal
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);//16진수로 표현 하라 {:x} x = hexadecimal
}