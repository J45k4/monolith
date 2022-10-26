use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    Dev
}