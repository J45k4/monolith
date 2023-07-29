use args::{Args, Commands};
use clap::Parser;
use monolith_core::Monolith;

mod args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run(args) => {
            if args.watch {
                println!("running in watch mode");
            }

            Monolith::new().add_script_path(&args.path).listen(8080).start().await;
        }
    }
}
