#[derive(Debug)]
struct Package{
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default()->Self {
        Self { weight:3.0 }
    }
}

fn main() {
    let p = Package::default();//사용자가 아무런 값을 생성하지 않고 사용을 원할때 초기값을 정한 default() 옵션도 생각하여 트래이트를 구성해 주는게 좋다
    println!("{:?}", p);
    println!("{}", p.weight);
}
