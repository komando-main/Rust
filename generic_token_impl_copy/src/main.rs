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

impl<A: Add<Output = A> + Copy, B, C> Lll<A, B, C> for Asd<A, B, C> {
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
	fn get_list(&self) -> [A; 2] {
        [self.q + self.w, self.w]
    }
	fn get_vec(&self) -> (Vec<&B>, &str) {
		(vec![&self.e, &self.r], "잘 할 수 있어")
	}
	fn get_tupl(&self) -> (&A,&A,&B,&B,&C,&C,f32,f64) {
		( &self.q, &self.w, &self.e, &self.r, &self.t, &self.y, 2.34f32, 4.67f64)
    }
}

fn main() {
	let c = Asd::new( 123u32, 456u32, -123i32, -456i32, String::from("Rust 최고"), String::from("재미 있는 Rust!"), ); println!("{:#?}", c);
	println!();
	println!("{}, {}", c.get_list()[0], c.get_list()[1]); println!("{:#?}", c.get_list());
	println!();
	println!( "{}, {}, {}", c.get_vec().0[0], c.get_vec().0[1], c.get_vec().1 );
	println!("{:#?}", c.get_vec());
	println!();
	println!( "{}, {}, {}, {}, {}, {}, {}, {}", c.get_tupl().0, c.get_tupl().1, c.get_tupl().2, c.get_tupl().3, c.get_tupl().4, c.get_tupl().5, c.get_tupl().6, c.get_tupl().7 );
    println!("{:#?}", 	c.get_tupl());
}