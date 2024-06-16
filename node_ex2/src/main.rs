#![allow(dead_code)]

use std::rc::Rc;
#[derive(Debug, Clone)]
struct Person{
    name:String,
    age:i32,
    next:Option<Rc<Person>>,
}

trait Asd {
    fn new(name:String, age:i32)->Rc<Self>;
    fn next(self, name:String, age:i32)->Rc<Person>;
}

impl Asd for Person {
    fn new(name:String, age:i32)->Rc<Self> {
        Self{
            name,
            age,
            next:None,
        }.into()
    }
    fn next(self, name:String, age:i32)->Rc<Person> {
        let mut p1 = Person::new(name, age);
        p1.next = Some(Rc::new(self));
        p1.into()
    }
}

fn main() {

    let mut p1 = Person::new("min".to_owned(), 39);
    p1 = p1.clone().next("ko".to_owned(), 38);
    println!("{:#?}", p1);
}
