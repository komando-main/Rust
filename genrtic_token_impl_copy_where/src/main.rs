use std::ops::Add;
#[derive(Debug, Clone, Copy)]
struct Asd<A, B, C> {
    q:A,
    w:A,
    e:B,
    r:B,
    t:C,
    y:C,
}
trait Lll<A, B, C> {
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self;
    fn get_list(&self) -> [A;3];
    fn get_vec(&self) -> (Vec<&B>, &str);
    fn get_tupl(&self) -> (&A, &A, &B, &B, &C, &C, f32, f64);
}
impl<A, B, C> Lll<A, B, C> for Asd<A, B, C> 
where A: Add<Output = A> + Copy,{//여러게 정의 해야 할때 ***T: Add<Output = T> + Copy, (다른 ops 정의) ***
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self {
        Self {
            q,
            w,
            e,
            r,
            t,
            y,
        }
    }

    fn get_list(&self) -> [A;3] {
        [self.q + self.w, self.q, self.w]
    }

    fn get_vec(&self) -> (Vec<&B>, &str) {
        (vec![&self.e, &self.r], "Fun rust")
    }

    fn get_tupl(&self) -> (&A, &A, &B, &B, &C, &C, f32, f64) {
        (&self.q, &self.w, &self.e, &self.r, &self.t, &self.y, 2.34f32, 4.67f64)
    }
}

fn main() {
    let c = Asd::new(123u32, 456u32, -123i32, -456i32, String::from("일단은 된다"), String::from("끝까지 한다"));
    println!("{:#?}", c);
    println!();
    println!("{1} + {2} = {0}", c.get_list()[0], c.get_list()[1], c.get_list()[2]);
    println!("{:#?}", c.get_list());
    println!();
    println!("{}, {}, {}", c.get_vec().0[0], c.get_vec().0[1], c.get_vec().1);
    println!("{:#?}", c.get_vec());
    println!();
    println!("{}, {}, {}, {}, {}, {}, {}, {}", c.get_tupl().0, c.get_tupl().1, c.get_tupl().2, c.get_tupl().3, c.get_tupl().4, c.get_tupl().5, c.get_tupl().6, c.get_tupl().7);
    println!("{:#?}", c.get_tupl());
}