use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    show: Option<Commands>,

    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Author,
    Card,
    Nano,
    Replace,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Directory {
        #[arg(short, long)]
        path: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    if let Some(directory) = cli.show {
        println!("{:?}", directory);
    }

    match cli.mode {
        Mode::Author => {
            println!("Author mode selected");
        }
        Mode::Card => {
            println!("Card mode selected");
        }
        Mode::Nano => {
            println!("Nano mode selected");
        }
        Mode::Replace => {
            println!("Replace mode selected");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run() {

    }
}
