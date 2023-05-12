fn main() {

    let a = 1;

    let b = 2;

    let c = add1(a, b);

    // let a2 = &a+&a1;

    // let b = &a.to_string();

    // let b1 = &a1.to_string();

    // let b2 = &a2.to_string();

    // let asd = [b,"+",b1,"=",b2].concat();

    // println!("{}", asd);

    println!("{}", c);
}
fn add1(i:i64, j:i64)->String{
    // let a=&i.to_string();
    // let b=&j.to_string();
    // let c = (&i+&j).to_string();
    // let d = [a,"+",b,"=",c].concat();
    // d
    [&i.to_string(),"+",&j.to_string(),"=",&(i+j).to_string()].concat()//concat() join() 둘다 가능
}