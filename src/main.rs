use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or_else(|| "");
    match command {
        "replace" => replace::init(),
        "card" => card::init(),
        "nano" => nano::init(),
        _ => println!("Unknown command"),
    }
}
