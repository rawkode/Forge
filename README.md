# Forge

A modern code forge built with Rust and jj (Jujutsu) version control.

## Overview

Forge is a next-generation code collaboration platform that emphasizes:

- **jj-native repositories** - Built around Jujutsu for superior version control
- **Private by default** - Security-first approach
- **Product-first monorepos** - Configuration via CUE schemas
- **Cedar authorization** - Fine-grained access control
- **Rust monolith** - High performance and reliability

## Features

- **Groups and Repositories** - Organize code under `git.<domain>` structure
- **Products** - Defined in `forge.cue` with dashboard views
- **Configuration Unification** - CUE-based configuration that unifies downward
- **Cedar Access Control** - Policy-based authorization with code owners
- **Typed Issues** - CUE-defined issue types with global numbering
- **Stacked Reviews** - PR-style reviews on jj changes with fast-forward merges
- **AsciiDoc Documentation** - Rich documentation with prebuilt releases
- **Global Search** - Tantivy-powered search with BM25 relevance

## Quick Start

1. **Install Dependencies**
   ```bash
   # Install jj (Jujutsu)
   cargo install --git https://github.com/martinvonz/jj.git jj-cli
   
   # Install CUE (for configuration)
   # See https://cuelang.org/docs/install/
   ```

2. **Configure Forge**
   ```bash
   cp forge.toml.example forge.toml
   # Edit forge.toml with your settings
   ```

3. **Run Forge**
   ```bash
   cargo run --bin forge
   ```

4. **Access the Application**
   - Web UI: http://localhost:3000
   - GraphQL API: http://localhost:3000/graphql
   - Health Check: http://localhost:3000/health

## Architecture

Forge is built as a Rust monolith with the following components:

- **forge-server** - Main HTTP server with Axum + GraphQL
- **forge-core** - Core domain types and utilities  
- **forge-auth** - OIDC authentication and JWT handling
- **forge-storage** - SQLite/libsql database layer
- **forge-vcs** - jj (Jujutsu) version control integration
- **forge-search** - Tantivy search engine
- **forge-policy** - Cedar authorization policies
- **forge-config** - CUE configuration management

## Development

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run with development mode
cargo run --bin forge -- --dev

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## Configuration

Forge uses CUE for configuration management. Configuration files:

- `forge.cue` - Product and repository configuration
- Repository-level `forge.cue` - Per-repository settings
- Cedar policies compiled from CUE at main HEAD

## License

Licensed under the GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later).
See [LICENSE](LICENSE) for details.

## Implementation Status

This is an early implementation of Forge v1. Current status:

- [x] Basic Rust workspace setup
- [x] Core type definitions
- [x] Server skeleton with Axum + GraphQL
- [x] Database migrations (SQLite)
- [x] Authentication framework (OIDC + JWT)
- [x] Basic health endpoints
- [ ] jj integration
- [ ] CUE configuration loading
- [ ] Cedar policy evaluation
- [ ] Search indexing
- [ ] Frontend (Astro + Vue)
- [ ] Full API implementation

See the implementation plan in the master document for detailed milestones.