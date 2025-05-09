use clap::{Parser, Subcommand};

/// A decentralized chat server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// initialize a new server
    Init {
        /// name of the server
        #[arg(short, long)]
        name: String,
        /// blurb of the server
        #[arg(short, long)]
        blurb: String,
    },
    /// run a server
    Run {
        
    },
    /// configure an existing server
    Config {
        /// name of the server
        #[arg(short, long)]
        name: String,
    },
}