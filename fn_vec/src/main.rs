fn string_collection(st:&str) -> Vec<String> {
    let mut c = vec![];
    let d = st.is_empty();    
    while !d {
        c.push(st.to_owned());
        break;
    }
    c
}
fn main() {
    let a = ["asd", "qwe", "zxc"];
    let b = string_collection(a[1]);
    
    println!("{:?}", b);
    println!("{:?}", a);
}