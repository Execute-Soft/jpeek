use crate::cli::{Cli, Commands};
use crate::jwt::JwtToken;
use colored::*;
use std::process;

pub struct App;

impl App {
    pub fn run(cli: Cli) {
        match &cli.command {
            Some(Commands::Decode {
                token,
                payload_only,
                header_only,
            }) => {
                Self::handle_decode(token, *payload_only, *header_only);
            }
            None => {
                Self::handle_direct_token(cli.token);
            }
        }
    }

    fn handle_decode(token: &str, payload_only: bool, header_only: bool) {
        match JwtToken::decode(token) {
            Ok(jwt) => {
                jwt.display(payload_only, header_only);
            }
            Err(e) => {
                eprintln!("{} {}", "❌ Error:".red().bold(), e);
                process::exit(1);
            }
        }
    }

    fn handle_direct_token(token: Option<String>) {
        if let Some(token) = token {
            Self::handle_decode(&token, false, false);
        } else {
            Self::show_usage();
        }
    }

    fn show_usage() {
        println!("{}", "🔐 JWT Token Decoder".bold().blue());
        println!("{}", "=".repeat(50).blue());
        println!("\n{}", "Usage:".bold());
        println!("  jpeek decode --token <JWT_TOKEN>");
        println!("  jpeek --token <JWT_TOKEN>");
        println!("\n{}", "Options:".bold());
        println!("  --payload-only    Show only the payload");
        println!("  --header-only     Show only the header");
        println!("\n{}", "Example:".bold());
        println!(
            "  jpeek decode --token eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
        );
    }
}
