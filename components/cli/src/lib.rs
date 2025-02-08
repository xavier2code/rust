use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    show: Option<Commands>,
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run() {

    }
}
