fn main() {
    let mut as1 = "     *     ".to_owned();
    
    let as2: usize = as1.len();
    let d = as1.trim().len();
    let mut b = 6;
    let mut c = 4;
    // println!("{b}");
    // println!("{c}");
    // println!("{}", as1.len());
    println!("{}", as2);
    println!("{}", d);
    while c != 0 {
        println!("{}", as1);
        as1.remove(0);
        as1.insert(c, '*');
        as1.insert(b, '*');
        b+=d;
        c-=d;
        if as1.trim().len() == b+c-d {
            // println!("!");
            as1.insert(b+c, ' ');
        }
    }
    // println!("{b}");
    // println!("{c}");
    // println!("{}", as1.len());
    // println!("{}", as1.trim().len());
    while c != 5 {
        as1.remove(c);
        as1.insert(0, ' ');
        as1.remove(b);
        b-=d;
        c+=d;
        println!("{}", as1);
    }
    // println!("{b}");
    // println!("{c}");
    println!("{}", as1.len());
    println!("{}", as1.trim().len());
    // println!("{}", as1);
}