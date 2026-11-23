use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: beuro <filename.beuro>");
        return;
    }

    let filename: &str = &args[1];
    println!("Compiling: {}", filename);
}
