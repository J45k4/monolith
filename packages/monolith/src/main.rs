use args::{Args, Commands};
use clap::Parser;

mod args;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Dev => {
            println!("Running in dev mode");
        }
    }
}
