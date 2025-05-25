# zk-email-prep

This project contains Rust code related to zero-knowledge (zk) email preparation and processing. It is intended as a starting point for building applications that use zero-knowledge proofs with email data.

## Structure
- `src/` - Main Rust source code for zk-email preparation.
- `Cargo.toml` - Rust project configuration and dependencies.

## Setup Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/zk-email-prep.git
   cd zk-email-prep
   ```
2. Install Rust: https://www.rust-lang.org/tools/install
3. Build the project:
   ```sh
   cargo build
   ```
4. Run tests:
   ```sh
   cargo test
   ```

## How it Works
This project uses Rust to prepare and process email data for zero-knowledge proofs. It includes modules for parsing email content, generating proofs, and verifying them. The core functionality is designed to be modular and extensible.

The main application is a web server built with Axum, which provides endpoints for registering and retrieving commitments. These commitments can be part of a larger system for handling zero-knowledge proofs or other cryptographic operations.

## Example Input/Output
### Input
A POST request to `/registry` with a JSON body like:
```json
{
  "commitment": "0x1234..."
}
```

### Output
The server responds with:
```json
{
  "commitment": "0x1234..."
}
```

### Input
A GET request to `/registry`.

### Output
The server responds with a list of commitments stored in the `commitments` file.

## Purpose
This project is a foundation for experimenting with zero-knowledge proofs and email data in Rust.
