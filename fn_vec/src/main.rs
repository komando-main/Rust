fn string_collection(st:&str) -> Vec<String> {
    let mut c = vec![];
    let d = st.is_empty();
    if !d {
        c.push(st.to_owned());
    }
    c
}

fn str_array(st:&[&str;3]) -> Vec<String> {
    let mut c = vec![];
    // println!("str_array 함수 안 {:?}", st);
    for a in st.iter() {
        // println!("{:?}", a.to_string());
        c.push(a.to_string());
    }
    c
}


fn main() {
    let a = ["asd", "qwe", "zxc"];
    let b = string_collection(a[1]);
    let c = str_array(&a);
    println!("{:?}", b);
    println!("{:?}", a);
    println!("{:?}", c);
}