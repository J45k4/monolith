use args::{Args, Commands};
use clap::Parser;
use log::LevelFilter;
use monolith_core::Monolith;
use simple_logger::SimpleLogger;

mod args;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .without_timestamps()
        .init()
        .unwrap();

    let args = Args::parse();

    match args.command {
        Commands::Run(args) => {
            if args.watch {
                println!("running in watch mode");
            }

            Monolith::new()
                .add("/", args.path)
                .listen(8080)
                .start().await;
        }
    }
}
