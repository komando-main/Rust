use clap::{Arg, ArgAction, Command};
fn main() {
    
    let marches = Command::new("echor")
    .version("0.1.0")
    .author("ken Youens-Clak <kyclark@gamil.com>")
    .about("Rust cersion of 'echo'")
    .arg(
        Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
        .short('n')
        .action(ArgAction::SetTrue)
        .help("Do not print newline"),
    )
    .get_matches();

    println!("{:#?}", marches);
}



// fn main() {
//     println!("{:#?}", std::env::args());
// }

// C:\kmj\Rust\echor [maincall|REBASE-i ≡ +7 ~74 -6 !]> cargo run -- -h
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target\debug\echor.exe -h`
// Rust cersion of 'echo'

// Usage: echor.exe

// Options:
//   -h, --help     Print help
//   -V, --version  Print version

// C:\kmj\Rust\echor [maincall|REBASE-i ≡ +7 ~74 -6 !]> cargo run -- --help
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target\debug\echor.exe --help`
// Rust cersion of 'echo'

// Usage: echor.exe [OPTIONS] <TEXT>...

// Arguments:
//   <TEXT>...  Input text

// Options:
//   -n             Do not print newline
//   -h, --help     Print help
//   -V, --version  Print version