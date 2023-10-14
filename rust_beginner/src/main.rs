fn multiply(x:i8, y:i16) -> i16 {
    println!("8bit                 {:08b}, {:o}, {x}", x, x);
    println!("i8 -> i16    {:016b}", x as i16);
    println!("16bit        {:016b}, {:x}, {y}", y, y);
    println!("x as i16 * y {:016b}", x as i16 * y);
    println!("x as i16 & y {:016b}", x as i16 & y);
    x as i16 * y // (x as i16) * y
}
fn main() {
    let mut x:i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();

    let x:i8 = 15;
    let y:i16 = 1000;
    println!("{x} {y}");

    println!("{x} * {y} = {}", multiply(x, y));

}
