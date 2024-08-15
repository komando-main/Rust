#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, Clone/*, Copy*/)]
struct Person{
    name:String,
    age:i32,
    // next:Option<Rc<Person>>,
    next: Option<Rc<RefCell<Person>>>,
}

trait Asd {
    fn new(name:String, age:i32)->Rc<RefCell<Self>>/*Rc<Self>*/;
    fn next(self, name:String, age:i32)->Rc<RefCell<Self>>/*Rc<Person>*/;
}

impl Asd for Person {
    fn new(name:String, age:i32)->Rc<RefCell<Self>>/*Rc<Self>*/ {
        Rc::new(RefCell::new(Self {
            name,
            age,
            next: None,
        }))
        /*Self{
            name,
            age,
            next:None,
        }.into()*/
    }
    fn next(self, name:String, age:i32)->Rc<RefCell<Person>>/*Rc<Person>*/ {
        let p1 = Person::new(name, age);
        p1.borrow_mut().next = Some(Rc::new(RefCell::new(self)));
        // p1.next = Some(self.clone().into());
        p1/*.into()*/
    }
}

fn main() {

    let p1 = Person::new("min".to_owned(), 39);
    println!("42: {:#?}", p1);
    p1.borrow().clone().next("ko".to_owned(), 38);
    // p1 = p1.clone().next("ko".to_owned(), 38);
    println!("45: {:#?}", p1.borrow().next);
}
