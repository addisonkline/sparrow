pub mod cli;
pub mod init;
pub mod server;
pub mod parser;
pub mod request;
pub mod response;
pub mod codes;
pub mod message;
pub mod user;
pub mod levels;

pub mod methods;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::init::init_server;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { name, blurb } => {
            init_server(name, blurb).await?;
        },
        Commands::Run {} => {
            
        }
    }


    Ok(())
}
