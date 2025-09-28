# GitHub Copilot Instructions for Forge

## Development Environment

This project uses `devenv.nix` for consistent development environment setup across all contributors. The environment includes:

- **Rust** (stable channel) with cargo, clippy, rustfmt, and rust-analyzer
- **Bun** for JavaScript/TypeScript development
- **Pre-commit hooks** for code quality

## Getting Started

1. **Install devenv**: Follow instructions at https://devenv.sh/getting-started/
2. **Enter the development shell**: `devenv shell`
3. **Available commands**:
   - `hello` - Welcome message and environment info
   - `format` - Format all code (Rust + JS/TS)
   - `lint` - Lint all code (Rust + JS/TS)
   - `test` - Run all tests (Rust + JS/TS)

## Project Structure Expectations

When working with GitHub Copilot on this project, please follow these conventions:

### Rust Development
- Use `cargo init --name <component-name>` for new Rust components
- Follow standard Rust project structure with `src/`, `tests/`, and `Cargo.toml`
- Use `cargo fmt` for formatting and `cargo clippy` for linting
- Write comprehensive tests in `tests/` directory or inline with `#[cfg(test)]`

### Bun/TypeScript Development  
- Use `bun init` for new TypeScript/JavaScript projects
- Follow modern TypeScript best practices
- Use Bun's built-in test runner for testing
- Prefer TypeScript over JavaScript for type safety

### Code Quality
- All code should pass the pre-commit hooks (formatting, linting, etc.)
- Write clear, self-documenting code with appropriate comments
- Follow conventional commit message format
- Include tests for all new functionality

## Copilot Usage Guidelines

When using GitHub Copilot in this project:

1. **Context Awareness**: Copilot should understand we're working with both Rust and Bun/TypeScript
2. **Environment Scripts**: Use the provided scripts (`format`, `lint`, `test`) for code quality
3. **Dependencies**: 
   - For Rust: Add dependencies to `Cargo.toml`
   - For Bun: Use `bun add <package>` or `bun install`
4. **Testing**: Always include tests for new features and bug fixes
5. **Documentation**: Update documentation when adding new features

## Common Tasks

### Adding a New Rust Component
```bash
devenv shell
mkdir rust-components/<component-name>
cd rust-components/<component-name>  
cargo init --name <component-name>
# Edit Cargo.toml, add dependencies, write code
cargo test
```

### Adding a New TypeScript Module
```bash
devenv shell
mkdir ts-modules/<module-name>
cd ts-modules/<module-name>
bun init -y
# Edit package.json, add dependencies, write code  
bun test
```

### Running Quality Checks
```bash
devenv shell
format  # Format all code
lint    # Lint all code  
test    # Run all tests
```

## CI/CD Integration

The project includes GitHub Actions workflows that:
- Validate the development environment setup
- Run tests on multiple platforms
- Check code formatting and linting
- Generate environment reports

## Contributing

1. Fork the repository
2. Set up the development environment with `devenv shell`
3. Create a feature branch
4. Make your changes following the guidelines above
5. Run `format`, `lint`, and `test` before committing
6. Submit a pull request

## Troubleshooting

- **Environment issues**: Run `devenv reload` to refresh the environment
- **Dependency conflicts**: Check both `Cargo.toml` and `package.json` for conflicts
- **Pre-commit failures**: Run `format` and `lint` scripts to fix most issues
- **Test failures**: Ensure all dependencies are installed with `cargo build` and `bun install`