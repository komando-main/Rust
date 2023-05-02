fn main() {
    
    let some_u8_value = 3;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    
    let a = 's';
    match a {
        's' => println!("{}",a),
        _ => (),
    }

    let b = "asd";
    match b {
        "asd" =>println!("{}", b),
        _ => (),
    }

    let x = vec!["asd", "qwe", "zxc"];
    for i in x.iter(){
        match *i {
            "zxc" => println!("{}", i),
            _ => (),
        }
    }

    let _n = "zxc".to_owned();
    let y = ["qwe".to_owned(),"asd".to_owned(),"zxc".to_owned()];
    for i in y.iter(){
        match i {
            _n => println!("{}", i),
        }
    }

}