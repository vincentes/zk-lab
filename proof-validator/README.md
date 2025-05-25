# Proof Validator

A full-stack application for validating zero-knowledge proofs, built with Rust and TypeScript.

## Project Structure

The project is organized into two main components:

- `rust/`: Backend service written in Rust
- `frontend/`: Web interface built with TypeScript and Vite

## Backend (Rust)

The backend service handles proof validation logic and provides API endpoints for the frontend.

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Setup

1. Navigate to the rust directory:
   ```bash
   cd rust
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the service:
   ```bash
   cargo run
   ```

## Frontend

The frontend provides a user-friendly interface for interacting with the proof validator.

### Prerequisites

- Node.js (LTS version)
- npm or yarn

### Setup

1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   # or
   yarn install
   ```

3. Start the development server:
   ```bash
   npm run dev
   # or
   yarn dev
   ```

## Development

### Backend Development

- The Rust backend uses Cargo for dependency management
- Source code is located in `rust/src/`
- Dependencies are managed in `Cargo.toml`

### Frontend Development

- Built with TypeScript and Vite
- Source code is located in `frontend/src/`
- Uses ESLint for code quality
- Dependencies are managed in `package.json`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

[Add your license information here]
