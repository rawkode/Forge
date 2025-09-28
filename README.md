# Forge

A modern development project supporting both Rust and Bun/TypeScript development.

## Quick Start

### Prerequisites

- [Nix](https://nixos.org/download.html) - Package manager and build system
- [devenv](https://devenv.sh/getting-started/) - Development environment manager

### Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/rawkode/Forge.git
   cd Forge
   ```

2. **Enter the development environment**:
   ```bash
   devenv shell
   ```

   This will automatically install and configure:
   - Rust (stable) with cargo, clippy, rustfmt, rust-analyzer
   - Bun for JavaScript/TypeScript development
   - Pre-commit hooks for code quality
   - Development scripts

3. **Verify the setup**:
   ```bash
   ./verify-setup.sh  # Run verification script
   hello              # Welcome message (inside devenv shell)
   ```

## Development

### Available Scripts

- `hello` - Display welcome message and environment info
- `format` - Format all code (Rust + JS/TS)
- `lint` - Lint all code (Rust + JS/TS)  
- `test` - Run all tests (Rust + JS/TS)

### Rust Development

Create a new Rust component:
```bash
cargo init --name my-component
cargo build
cargo test
```

### Bun/TypeScript Development

Create a new TypeScript project:
```bash
bun init -y
bun install
bun test
```

## Code Quality

This project uses pre-commit hooks to maintain code quality:

- **Rust**: rustfmt, clippy
- **General**: YAML/JSON/TOML validation, trailing whitespace removal
- **Nix**: nixpkgs-fmt

Run quality checks manually:
```bash
format  # Format all code
lint    # Lint all code
test    # Run all tests
```

## GitHub Copilot

This project is optimized for GitHub Copilot development. See [`.github/copilot-instructions.md`](.github/copilot-instructions.md) for detailed guidelines on using Copilot effectively in this environment.

## CI/CD

GitHub Actions workflows automatically:
- Validate the development environment
- Test Rust and Bun functionality  
- Run code quality checks
- Generate environment reports

## Contributing

1. Fork the repository
2. Set up the development environment: `devenv shell`
3. Create a feature branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Run quality checks: `format && lint && test`
6. Commit your changes: `git commit -m "feat: add my feature"`
7. Push to your fork: `git push origin feature/my-feature`
8. Create a Pull Request

## License

This project is licensed under the GNU Affero General Public License v3.0 - see the [LICENSE](LICENSE) file for details.