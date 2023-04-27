#[derive(Debug)]
struct Asd<T> {
    qwe:Yyy<T>,
    iop:Yyy<T>
}
#[derive(Debug)]
enum Yyy<T> {
    Qq(T),
    Ww(T),
}

trait Jkl<T> {
    fn new(a:T, b:T) -> Self;
    fn get_ww(&self) -> (&Yyy<T>, &Yyy<T>);
    fn get_a(&self) -> (&T,&T);
}

impl<T> Jkl<T> for Asd<T>{
    fn new(a:T, b:T) -> Self {
        Self {
            qwe: Yyy::Qq(a),
            iop: Yyy::Ww(b),
        }
    }
    
    fn get_ww(&self) -> (&Yyy<T>, &Yyy<T>){
        (&self.qwe, &self.iop)
    }

    fn get_a(&self) -> (&T,&T) {
        (
            match &self.qwe {
                self::Yyy::Qq(a) => a,
                _ => todo!(),
                //self::Yyy::Ww(_) => todo!(),
            },
            match &self.iop {
                self::Yyy::Ww(a) => a,
                _=> todo!(),
                //self::Yyy::Qq(_) => todo!(),
            }
        )
        
    }
}

fn main() {
    
    let a = Asd::new(3u8, 7u8);

    let (c, d) = a.get_ww();

    let (f, e) = a.get_a();
    
    println!("{:?}", a);

    println!("{:?}", a.get_ww());

    println!("{:?}", c);

    println!("{:?}", d);

    println!("{}, {}", f, e);
    
}