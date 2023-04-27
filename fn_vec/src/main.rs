fn string_collection(st:&str) -> Vec<String> {
    let mut c = vec![];
    let d = st.is_empty();    
    while !d {
        c.push(st.to_owned());
        break;
    }
    c
}