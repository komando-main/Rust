use std::ops::Add;

#[derive(Debug)]
struct Asd<A: Add<Output = A>, B, C> {
    q: A,
    w: A,
    e: B,
    r: B,
    t: C,
    y: C,
}

trait Lll<A, B, C> {
    fn new(q: A, w: A, e: B, r: B, t: C, y: C) -> Self;
    fn get_list(&self) -> [A; 2];
    fn get_vec(&self) -> (Vec<&B>, &str);
    fn get_tupl(&self) -> (&A, &A, &B, &B,&C,&C,f32,f64);
}
// 참조로 토큰 사용법좀 찿아 봐야 하나.. 이건 또 얼마나 걸리려나 dlfeks 일단 찾아야지 방법이 있을꺼야.. 없을수도 있것지만...
impl<A: Add<Output = A> + Clone, B, C> Lll<A, B, C> for Asd<A, B, C> { //A: Add<Output = A> + Clone 이개 핵심이내.. 아 넘기기 밥통아... 여러개 만들어야 하는 불편함이 있구만...
    fn new(q: A, w: A, e: B, r: B, t: C, y: C) -> Self {
        Self {
            q,
            w,
            e,
            r,
            t,
            y,
        }
    }

    fn get_list(&self) -> [A; 2] {//클론 으로 넘겨라!! 핵심이다... 아 여테 클론 쓰질 않은 티가 확나내... 참조로 무조건 해결 할라고 들었으니 안되지..
        // let a = self.q.clone() + self.w.clone();
        // [a,self.w.clone()]
        [self.q.clone() + self.w.clone(), self.w.clone()]
    }

    fn get_vec(&self) -> (Vec<&B>, &str) {
        (vec![&self.e, &self.r], "잘 할 수 있어")
    }

    fn get_tupl(&self) -> (&A,&A,&B,&B,&C,&C,f32,f64) {
        (
            &self.q,
            &self.w,
            &self.e,
            &self.r,
            &self.t,
            &self.y,
            2.34f32,
            4.67f64,
        )
    }
}

fn main() {
    let c = Asd::new(
        123u32,
        456u32,
        -123i32,
        -456i32,
        String::from("Rust 최고"),
        String::from("재미 있는 Rust!"),
    );
    println!("{:#?}", c);
    println!();
    println!("{}, {}", c.get_list()[0], c.get_list()[1]);
    println!("{:#?}", c.get_list());
    println!();
    println!(
        "{}, {}, {}",
        c.get_vec().0[0], c.get_vec().0[1], c.get_vec().1
    );
    println!("{:#?}", c.get_vec());
    println!();
    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}",
        c.get_tupl().0,
        c.get_tupl().1,
        c.get_tupl().2,
        c.get_tupl().3,
        c.get_tupl().4,
        c.get_tupl().5,
        c.get_tupl().6,
        c.get_tupl().7
    );
    println!("{:#?}", c.get_tupl());
}