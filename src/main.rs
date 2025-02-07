use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("");
    match command {
        "author" => author::init(),
        _ => println!("Unknown command"),
    }
}