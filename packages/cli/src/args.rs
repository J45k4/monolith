use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    Run(RunArgs)
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    pub path: String,
    #[clap(short, long, default_value = "false")]
    pub watch: bool
}