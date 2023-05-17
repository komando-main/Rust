use std::ops::Add;
#[derive(Debug)]
struct Asd<A: Add<Output = A>, B, C> {// 더하기 옵션 설정 스트럭터에 설정 알고나면 정말 아무것도 아닌거다 단지 혼자 하려고 하니 어려운것 뿐이지
	q: A,
	w: A,
	e: B,
	r: B,
	t: C,
	y: C,
}

trait Lll<A, B, C> {
	fn new(q: A, w: A, e: B, r: B, t: C, y: C) -> Self;
	fn get_list(&self) -> [A; 2];//더하기 옵션은 참조값으로 더할수 없다 그러기에 카피하여 더한값을 넘김으로 보낸다
	fn get_vec1(&self) -> (Vec<A>, &str);//백터또한 위와 동일하게 카피하여 더한값을 넘김으로 보넨다
	fn get_vec(&self) -> (Vec<&B>, &str);
    fn get_tupl(&self) -> (&A, &A, &B, &B,&C,&C,f32,f64);
}

impl<A: Add<Output = A> + Copy, B, C> Lll<A, B, C> for Asd<A, B, C> {//impl블록에 스트럭처에 명시한 A제네릭을 카피도 같이 사용한다고 명시한다 
	fn new(q: A, w: A, e: B, r: B, t: C, y: C) -> Self {
		 Self {
			q,//인자 값의 이름이 스트럭처의 바인드 명과 같다면 q:q 라고 명시 할 필요 없다
			w,
			e,
			r,
			t,
			y,
		}
	}
	fn get_list(&self) -> [A; 2] {//카피가 되었기에 넘길수 있다 참조로는 넘길수 없다!
        [self.q + self.w, self.w]// .copy() 가 암묵적임으로 명시 할필요 없다 단 정의는 해줘야 하며 참조로 보내면 안된다
    }
	fn get_vec(&self) -> (Vec<&B>, &str) {
		(vec![&self.e, &self.r], "잘 할 수 있어")
	}
	fn get_vec1(&self) -> (Vec<A>, &str) {
		(vec![self.q + self.w, self.q, self.w,], "이것도 가능하다")
	}
	fn get_tupl(&self) -> (&A,&A,&B,&B,&C,&C,f32,f64) {
		( &self.q, &self.w, &self.e, &self.r, &self.t, &self.y, 2.34f32, 4.67f64)
    }
}

fn main() {
	let c = Asd::new( 123u32, 456u32, -123i32, -456i32, String::from("Rust 최고"), String::from("재미 있는 Rust!"));
    println!("{:#?}", c);
    println!();
	println!("123 + 456 = {}, {}", c.get_list()[0], c.get_list()[1]); println!("{:#?}", c.get_list());
	println!();
	println!( "{}, {}, {}", c.get_vec().0[0], c.get_vec().0[1], c.get_vec().1 );
	println!("{:#?}", c.get_vec());
	println!();
	println!("{1} + {2} = {0}, {3}", c.get_vec1().0[0], c.get_vec1().0[1], c.get_vec1().0[2], c.get_vec1().1);
	println!("{:#?}", c.get_vec1());
	println!();
	println!( "{}, {}, {}, {}, {}, {}, {}, {}", c.get_tupl().0, c.get_tupl().1, c.get_tupl().2, c.get_tupl().3, c.get_tupl().4, c.get_tupl().5, c.get_tupl().6, c.get_tupl().7 );
    println!("{:#?}", 	c.get_tupl());
}