use cli;
fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("nano", sub_matches)) => {
            println!("nano {}", sub_matches.get_one::<String>("name").unwrap());
        }
        _ => {}
    }
}