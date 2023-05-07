fn main() {
    let mut as1 = "     *     ".to_owned();

    let mut b = 6;
    let mut c = 4;

    while c != 0 {
        println!("{}", as1);
        as1.remove(0);
        as1.insert(c, '*');
        as1.insert(b, '*');
        b+=1;
        c-=1;
    }
    while c != 5 {
        as1.remove(c);
        as1.insert(0, ' ');
        as1.remove(b);
        b-=1;
        c+=1;
        println!("{}", as1);
    }
}