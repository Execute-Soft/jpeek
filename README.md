# 🔐 jpeek - JWT Token Decoder

[![Support me](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/moshedulmunna)

[![Rust](https://img.shields.io/badge/Rust-1.86.0+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/jpeek)](https://crates.io/crates/jpeek)

## ✨ Features

- 🔍 **Easy JWT Decoding**: Decode JWT tokens with a simple command
- 🎨 **Beautiful Output**: Color-coded and formatted JSON output
- ⏰ **Timestamp Display**: Human-readable expiration and issuance dates
- 📋 **Flexible Viewing**: View header, payload, or both separately
- 🚀 **Fast & Lightweight**: Built in Rust for optimal performance
- 🛠️ **Cross-platform**: Works on macOS, Linux, and Windows

## 📦 Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.86.0 or later)

### Install from Source

1. Clone the repository:

```bash
git clone https://github.com/Execute-Soft/jpeek.git
cd jpeek
```

2. Build and install:

```bash
cargo install --path .
```

The `jpeek` command will be available globally in your system.

### Alternative Installation Methods

#### Build Locally (without installing)

```bash
cargo build --release
./target/release/jpeek --help
```

#### Manual Installation

```bash
cargo build --release
cp target/release/jpeek /usr/local/bin/
```

## 🚀 Usage

### Basic Usage

```bash
# Decode a JWT token
jpeek -t "your.jwt.token.here"

# Or use the decode subcommand
jpeek decode -t "your.jwt.token.here"
```

### Advanced Usage

```bash
# Show only the payload
jpeek decode -t "your.jwt.token.here" -p

# Show only the header
jpeek decode -t "your.jwt.token.here" -H

# Get help
jpeek --help
jpeek decode --help
```

### Example Output

```bash
jpeek decode -t "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
```

**Output:**

```
🔐 JWT Token Decoder
==================================================

📋 Header:
{
  "alg": "HS256",
  "typ": "JWT"
}

📄 Payload:
{
  "sub": "1234567890",
  "name": "John Doe",
  "iat": 1516239022
}

🔒 Signature:
SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c

📅 Issued: 2018-01-18 21:30:22 UTC
```

## 🛠️ Command Line Options

### Global Options

- `-t, --token <TOKEN>` - JWT token to decode
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Decode Command Options

- `-t, --token <TOKEN>` - JWT token to decode (required)
- `-p, --payload-only` - Show only the payload
- `-H, --header-only` - Show only the header
- `-h, --help` - Print help information

## 🔧 Development

### Prerequisites

- Rust 1.86.0 or later
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:

```bash
git clone https://github.com/Execute-Soft/jpeek.git
cd jpeek
```

2. Build the project:

```bash
cargo build
```

3. Run tests:

```bash
cargo test
```

4. Run the application:

```bash
cargo run -- -t "your.jwt.token.here"
```

### Project Structure

```
jpeek/
├── src/
│   ├── main.rs      # Application entry point
│   ├── app.rs       # Main application logic
│   ├── cli.rs       # Command-line interface definitions
│   └── jwt.rs       # JWT token parsing and display logic
├── Cargo.toml       # Project dependencies and metadata
└── README.md        # This file
```

## 📋 Dependencies

- **clap** - Command-line argument parsing
- **colored** - Terminal color formatting
- **base64** - Base64 decoding
- **serde_json** - JSON parsing and formatting
- **serde** - Serialization/deserialization
- **chrono** - Date and time handling

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Morshedul Islam Munna**

- Email: morshedulmunna1@gmail.com
- GitHub: [@morshedulmunna](https://github.com/morshedulmunna)
- LinkedIn: [Morshedul Islam Munna](https://www.linkedin.com/in/morshedulmunna/)

## 🏢 Organization

**Execute-Soft**

- GitHub: [@Execute-Soft](https://github.com/Execute-Soft)

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Inspired by the need for a simple JWT token decoder
- Thanks to the Rust community for excellent libraries

## 📈 Roadmap

- [ ] Add JWT token validation
- [ ] Support for different JWT algorithms
- [ ] Interactive mode for token input
- [ ] Export decoded data to different formats
- [ ] Add support for JWT token creation

---

⭐ If you find this tool useful, please give it a star on [GitHub](https://github.com/Execute-Soft/jpeek)!
