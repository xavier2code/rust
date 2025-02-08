use clap::Command;

pub fn cli() -> Command {
    Command::new("rustlet")
        .about("A CLI tool for managing Rust projects.")
        .author("xavier2code")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .color(clap::ColorChoice::Auto)
        .version("0.1.0")
        .subcommand(
            Command::new("nano")
                .about("Create a new Rust project.")
                .arg(
                    clap::Arg::new("name")
                        .help("Name of the project.")
                        .required(true),
                )
                .arg_required_else_help(true)
            ,
        )
        .subcommand(
            Command::new("card")
                .about("Build the Rust project.")
        )
}