#![allow(unused_variables)]
use std::any::type_name;//이걸 써줘야 type_name::<T>() 이걸쓸 수 있다

fn type_of<T>(_: &T) -> &str {//제네릭으로 바로 보낼깨 인자에 바인드가 1개인 경우 _ (이름을 지정하지 않음) 으로 보낸다 도있지만 타입을 알아서 추론 하라 라는 뜻도 된다
    type_name::<T>()//반환 타입 &str
}


fn main() {
    
    let a = &1;
    let a1 = 5_u8;
    println!("{} {}", type_of(&a1), type_of(&&a1));
    let b = &2;

    let b12 = str1();
    println!("{:?}", b12);
    // println!("{:?}", &b12[..2]);

    let c = add1(a, b);
    let c1 = 123u8;
    println!("c1 좀 보자 {}", type_of(&c1));
    let c2 = c1.to_string().as_str();
    // let c3 = c1.as_str(); 바로 바꿀수 없다 to_string()을 거처서 바꿔야 한다!
    // let d = c.as_str();

    // let a2 = &a+&a1;

    // let b = &a.to_string();

    // let b1 = &a1.to_string();

    // let b2 = &a2.to_string();

    // let asd = [b,"+",b1,"=",b2].concat();

    // println!("{}", asd);

    println!("매인 안 {:p} {}, {}", a, type_of(&a), type_of(&*a));
    
    println!("매인 안 {:p} {}", b, type_of(&b));
    println!("{}", type_of(&c).trim().len() - 6);
    println!("문자열 자르기전 {}, {:?}, {}", c, c, type_of(&c));
    println!("문자열 자르기후 {}, {:?}, {}", c, c, &type_of(&c)[15..]);
    // println!("매인 안 {:p} {}", b, type_of(&d));
    println!("{:?}", c);
    
}
// fn add1(i:&u8, j:&u8)->&'static str{
fn add1(i:&u8, j:&u8)->String{
    let s = &i;
    let f = &j;
    println!("함수 안 s {}", type_of(&s));
    println!("함수 안 f {}", type_of(&f));
    println!("함수 안 i {:p}, {}", i, type_of(&i));
    println!("함수 안 j {:p}, {}", j, type_of(&j));
    // let a=&i.to_string();
    // let b=&j.to_string();
    // let c = (&i+&j).to_string();
    // let d = [a,"+",b,"=",c].concat();
    // d
    // [&i.to_string(), "+", &j.to_string(), "=", &(i+j).to_string()].concat()//concat() join() 둘다 가능
    // println!("함수 안 return {} {:p}, {}", [&i.to_string(), " + ", &j.to_string(), " = ", &(*i + *j).to_string()].concat().as_str(), &[&i.to_string(), " + ", &j.to_string(), " = ", &(*i + *j).to_string()].concat().as_str(), type_of(&[&i.to_string(), " + ", &j.to_string(), " = ", &(*i + *j).to_string()].concat().as_str()));
    // [&i.to_string(), " + ", &j.to_string(), " = ", &(*i + *j).to_string()].concat().as_str()
    // [i.to_string(), "+".to_owned(), j.to_string(), "=".to_owned(), (*i + *j).to_string()].join("")
    // [i.to_string(), "+".to_owned(), j.to_string(), "=".to_owned(), (i + j).to_string()].join("")
    // [&i.to_string(), " + ", &j.to_string(), " = ", &(*i + *j).to_string()].concat().as_str() //&str 인대 왜 안되지?
    // "test" //왜 이것 만 되는거지????
    [&i.to_string(), " + ", &j.to_string(), " = ", &(*i + *j).to_string()].concat()
    // println!("함수 안 {:?}", asd1);
}

fn str1()->&'static str{
    "test"
}