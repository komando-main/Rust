#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
use std::sync::{Arc, Mutex};
use std::any::type_name;

#[derive(Debug, Clone)]
struct Table<A, B, C> {
    q:Option<Box<A>>,
    w:Option<Box<A>>,
    e:Option<Box<B>>,
    r:Option<Box<B>>,
    t:Option<Box<C>>,
    y:Option<Box<C>>,
    up: Option<Arc<Mutex<Table<A, B, C>>>>,//Arc는 'Atomic Reference Counting’의 약자로, 여러 스레드에서 안전하게 공유할 수 있는 참조 카운팅 포인터
    down: Option<Arc<Mutex<Table<A, B, C>>>>,
}

impl<A, B, C> PartialEq for Table<A, B, C>
where
A: PartialEq + std::fmt::Debug + Clone,
B: PartialEq + std::fmt::Debug + Clone,
C: PartialEq + std::fmt::Debug + Clone,
{
    fn eq(&self, other: &Self) -> bool {//이전생성과 지금 생성의 주소가 같은지 비교문
        self.q == other.q && self.w == other.w && self.e == other.e && self.r == other.r && self.t == other.t && self.y == other.y
    }
}

trait Table_fn<A, B, C>:std::fmt::Debug{
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self where Self: Sized;
    fn set_table(&mut self, q:A, w:A, e:B, r:B, t:C, y:C);
    fn get_start_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    fn get_down_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    fn get_down_twostep_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    // fn get_up_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]);
    //fn get_q(&self) -> Option<Box<A>>;
}

impl<A, B, C> Table_fn<A, B, C> for Table<A, B, C> 
where 
A:std::fmt::Debug + Clone, 
B:std::fmt::Debug + Clone, 
C:std::fmt::Debug + Clone,
Table<A, B, C>: PartialEq,
{
    fn new(q:A, w:A, e:B, r:B, t:C, y:C) -> Self where Self: Sized {
        Self {
            q:Some(Box::new(q)),
            w:Some(Box::new(w)),
            e:Some(Box::new(e)),
            r:Some(Box::new(r)),
            t:Some(Box::new(t)),
            y:Some(Box::new(y)),
            up:None,
            down:None,
        }
    }

    fn set_table(&mut self, q: A, w: A, e: B, r: B, t: C, y: C) {
        let new_table = Arc::new(Mutex::new(Self::new(q, w, e, r, t, y)));
        if let Some(down) = &self.down {
            new_table.lock().unwrap().up = Some(down.clone());
        }
        self.down = Some(new_table);
    }

    // fn get_q(&self) -> Option<Box<A>> {
    //     self.q.clone()
    // }

    fn get_start_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
        ([self.q.clone(), self.w.clone()], [self.e.clone(),self.r.clone()], [self.t.clone(), self.y.clone()])
    }

    // fn get_down_table(&self) -> ([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]) {
    //     if let Some(down) = &self.down {
    //         let down_table = down.lock().unwrap();
    //         return ([down_table.q.clone(), down_table.w.clone()], [down_table.e.clone(), down_table.r.clone()], [down_table.t.clone(), down_table.y.clone()]);
    //     }
    //     panic!("Down table is None");
    // }
    fn get_down_twostep_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
        if let Some(down) = &self.down {
            let down_table = down.lock().unwrap();
            if let Some(down_down) = &down_table.down {
                let down_down_table = down_down.lock().unwrap();
                return ([down_down_table.q.clone(), down_down_table.w.clone()], [down_down_table.e.clone(), down_down_table.r.clone()], [down_down_table.t.clone(), down_down_table.y.clone()]);
            }
        }
        panic!("Down table is None");
    }
    fn get_down_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
        if self.down.is_some() {
            let down_table = self.down.as_ref().unwrap().lock().unwrap();
            return ([down_table.q.clone(), down_table.w.clone()], [down_table.e.clone(), down_table.r.clone()], [down_table.t.clone(), down_table.y.clone()]);
        }
        panic!("Down table is None");
    }
    // fn get_down_twostep_table(&self)->([Option<Box<A>>;2], [Option<Box<B>>;2], [Option<Box<C>>;2]){
    //     let down_table;
    //     if self.down.is_some() {
    //         down_table = self.down.as_ref().unwrap().lock().unwrap();
    //         if down_table.down.is_some(){
    //             down_table = down_table.down.as_ref().unwrap().lock().unwrap();
    //             return ([down_table.q.clone(), down_table.w.clone()], [down_table.e.clone(), down_table.r.clone()], [down_table.t.clone(), down_table.y.clone()]);
    //         }
    //     }
    //     panic!("Down table is None");
    // }

}

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    let mut start: Box<dyn Table_fn<_, _, _>> = Box::new(Table::new(123_u32, 456_u32, -123_i32, -456_i32, String::from("일단은 된다"), String::from("끝까지 한다")));
    start.set_table(789u32, 012u32, 345i32, 678i32, String::from("foo"), String::from("bar"));
    start.set_table(1, 2, 3, 4, String::from("5"), String::from("6"));
    start.set_table(0, 9, 8, 7, "6".to_owned(), "5".to_owned());
    start.set_table(12, 9, 8, 7, "6".to_owned(), "5".to_owned());
    start.set_table(23, 9, 8, 7, "6".to_owned(), "5".to_owned());
    println!("{:#?}", start);
    println!();
    println!("start.get_start_table().2[0].as_ref().unwrap()\n {:?}\n", start.get_start_table().2[0].as_ref().unwrap());
    println!("\nstart.get_start_table()\n {:?}\n", start.get_start_table());
    println!();
    println!("\nstart.get_down_table()\n {:?}\n", start.get_down_table());
    println!();
    println!("\nstart.get_down_twostep_table()\n {:?}\n", start.get_down_twostep_table());
}