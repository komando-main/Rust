fn main() {
    for i in 0..=100 {
        let msg = match i {
            // n => "되냐?".to_owned() + &n.to_string(),
            n if n % 3 == 0 => "Fizz ".to_owned() + &n.to_string(),
            n if n % 5 == 0 => "Buzz ".to_owned() + &n.to_string(),
            n if n % 15 == 0 => "FizzBuzz ".to_owned() + &n.to_string(),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}