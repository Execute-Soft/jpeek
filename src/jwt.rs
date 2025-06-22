use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono;
use colored::Colorize;
use serde_json::Value;

#[derive(Debug)]
pub struct JwtToken {
    pub header: Value,
    pub payload: Value,
    pub signature: String,
}

impl JwtToken {
    pub fn decode(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = token.split('.').collect();

        if parts.len() != 3 {
            return Err("Invalid JWT token format. Expected 3 parts separated by dots.".into());
        }

        let header = Self::decode_base64_json(parts[0])?;
        let payload = Self::decode_base64_json(parts[1])?;
        let signature = parts[2].to_string();

        Ok(JwtToken {
            header,
            payload,
            signature,
        })
    }

    fn decode_base64_json(encoded: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let decoded = URL_SAFE_NO_PAD.decode(encoded)?;
        let json_str = String::from_utf8(decoded)?;
        let json_value = serde_json::from_str(&json_str)?;
        Ok(json_value)
    }

    pub fn display(&self, payload_only: bool, header_only: bool) {
        if !header_only {
            println!("{}", "🔐 JWT Token Decoder".bold().blue());
            println!("{}", "=".repeat(50).blue());
        }

        if !payload_only {
            println!("{}", "\n📋 Header:".bold().green());
            println!(
                "{}",
                serde_json::to_string_pretty(&self.header).unwrap().green()
            );
        }

        if !header_only {
            println!("{}", "\n📄 Payload:".bold().yellow());
            println!(
                "{}",
                serde_json::to_string_pretty(&self.payload)
                    .unwrap()
                    .yellow()
            );

            println!("{}", "\n🔒 Signature:".bold().red());
            println!("{}", self.signature.red());

            // Display common JWT claims in a more readable format
            self.display_timestamps();
        }
    }

    fn display_timestamps(&self) {
        if let Some(exp) = self.payload.get("exp") {
            if let Some(exp_num) = exp.as_u64() {
                let exp_date = chrono::DateTime::from_timestamp(exp_num as i64, 0);
                if let Some(date) = exp_date {
                    println!(
                        "{}",
                        format!("\n⏰ Expires: {}", date.format("%Y-%m-%d %H:%M:%S UTC")).cyan()
                    );
                }
            }
        }

        if let Some(iat) = self.payload.get("iat") {
            if let Some(iat_num) = iat.as_u64() {
                let iat_date = chrono::DateTime::from_timestamp(iat_num as i64, 0);
                if let Some(date) = iat_date {
                    println!(
                        "{}",
                        format!("📅 Issued: {}", date.format("%Y-%m-%d %H:%M:%S UTC")).cyan()
                    );
                }
            }
        }
    }
}
