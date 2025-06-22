use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jpeek")]
#[command(about = "A simple tool to peek into your JWT token & Decode it")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// JWT token to decode
    #[arg(short, long)]
    pub token: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Decode a JWT token
    Decode {
        /// JWT token to decode
        #[arg(short, long)]
        token: String,

        /// Show only the payload
        #[arg(short, long)]
        payload_only: bool,

        /// Show only the header
        #[arg(short = 'H', long)]
        header_only: bool,
    },
}
