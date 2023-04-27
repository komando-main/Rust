#[derive(Debug)]
struct Asd<T> {
    qq: Rt<T>,
}
#[derive(Debug)]
enum Rt<T> {
    Vas(T),
}

trait Yuu<T> {
    fn new(x:T) -> Self;
    fn get_enum_rt(&self) -> &T;
}

impl<T> Yuu<T> for Asd<T> {
    fn new(x:T) -> Self {
        Self {
            qq: Rt::Vas(x),
        }
    }

    fn get_enum_rt(&self) -> &T {
        match &self.qq {
            self::Rt::Vas(a) => a,
        }
    }
}

fn main() {
    let lll = Asd::new(8u8);
    let ppp = lll.get_enum_rt();
    println!("{:?}", lll.qq);
    println!("{}", ppp);
}