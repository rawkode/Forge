#!/usr/bin/env bash

# Forge Development Environment Verification Script
# This script verifies that the development environment is properly set up

set -e

echo "🔍 Verifying Forge Development Environment..."
echo

# Check if we're in a devenv shell
if [[ -n "$DEVENV_PROFILE" ]]; then
    echo "✅ Running in devenv shell"
else
    echo "⚠️  Not running in devenv shell - run 'devenv shell' first"
    echo "   You can still verify tools are available globally..."
    echo
fi

# Check Rust tools
echo "📦 Checking Rust tools..."
if command -v rustc &> /dev/null; then
    echo "  ✅ rustc: $(rustc --version)"
else
    echo "  ❌ rustc not found"
    exit 1
fi

if command -v cargo &> /dev/null; then
    echo "  ✅ cargo: $(cargo --version)"
else
    echo "  ❌ cargo not found"
    exit 1
fi

if command -v rustfmt &> /dev/null; then
    echo "  ✅ rustfmt: $(rustfmt --version)"
else
    echo "  ❌ rustfmt not found"
fi

if command -v clippy-driver &> /dev/null; then
    echo "  ✅ clippy: available"
else
    echo "  ❌ clippy not found"
fi

# Check Bun
echo
echo "🟨 Checking Bun..."
if command -v bun &> /dev/null; then
    echo "  ✅ bun: $(bun --version)"
else
    echo "  ❌ bun not found (this is expected if not in devenv shell)"
fi

# Test creating a sample Rust project
echo
echo "🦀 Testing Rust project creation..."
mkdir -p temp-rust-test
cd temp-rust-test
cargo init --name test-project --bin --quiet
if cargo check --quiet; then
    echo "  ✅ Rust project creation and compilation works"
else
    echo "  ❌ Rust project compilation failed"
    exit 1
fi
cd ..
rm -rf temp-rust-test

# Check for devenv files
echo
echo "📁 Checking devenv configuration..."
if [[ -f "devenv.nix" ]]; then
    echo "  ✅ devenv.nix found"
else
    echo "  ❌ devenv.nix not found"
    exit 1
fi

if [[ -f "devenv.yaml" ]]; then
    echo "  ✅ devenv.yaml found"
else
    echo "  ❌ devenv.yaml not found"
    exit 1
fi

# Check for GitHub workflow
echo
echo "🔄 Checking GitHub Actions workflow..."
if [[ -f ".github/workflows/copilot-environment.yml" ]]; then
    echo "  ✅ Copilot environment workflow found"
else
    echo "  ❌ Copilot environment workflow not found"
    exit 1
fi

# Check documentation
echo
echo "📚 Checking documentation..."
if [[ -f "README.md" ]]; then
    echo "  ✅ README.md found"
else
    echo "  ❌ README.md not found"
fi

if [[ -f ".github/copilot-instructions.md" ]]; then
    echo "  ✅ Copilot instructions found"
else
    echo "  ❌ Copilot instructions not found"
fi

echo
echo "🎉 Environment verification complete!"
echo
echo "Next steps:"
echo "  1. Run 'devenv shell' to enter the development environment"
echo "  2. Inside the devenv shell, you can use:"
echo "     - hello    (welcome message)"
echo "     - format   (format code)"
echo "     - lint     (lint code)"
echo "     - test     (run tests)"
echo "  3. See README.md for detailed usage instructions"
echo