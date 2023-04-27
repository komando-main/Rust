use num::complex::Complex;//카고 빌드패키지 활용 밥업 Cargo.toml 파일 확인

fn main() {
    let a = Complex{re:2.1_f32, im:-1.2_f32};
    let b = Complex::new(11.1_f32, 22.2_f32);
    let result = a+b;
    println!("{} + {}i", result.re, result.im); //13.200001 + 21i 잘된다 빌드 패키지 받아야 한다
}