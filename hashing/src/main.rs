
mod hashing;
use hashing::sha256_hash;

// example of hashing 'hello world'
// fn main() {
//     println!("Input: 'hello world'");
//     let hash = sha256_hash("hello world");
//     println!("SHA256 Hash: {}", hash);
// }


// example of hashing any input from command line
// use `cargo build --release` to build the project
// use std::env;
// use std::process;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprint!("Usage: {} <input_string>", args[0]);
//         process::exit(1);
//     }

//     let input = &args[1];
//     let hash = sha256_hash(input);
//     println!("Input: '{}'", input);
//     println!("SHA256 Hash: {}", hash);
// }


