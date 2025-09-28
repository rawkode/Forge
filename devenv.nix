{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    curl
    wget
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  languages.javascript = {
    enable = true;
    bun.enable = true;
    bun.install.enable = true;
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  scripts.format.exec = ''
    echo "Running formatters..."
    if [ -f Cargo.toml ]; then
      cargo fmt
    fi
    if [ -f package.json ] || [ -f bun.lockb ]; then
      bun run format 2>/dev/null || echo "No format script found in package.json"
    fi
  '';

  scripts.lint.exec = ''
    echo "Running linters..."
    if [ -f Cargo.toml ]; then
      cargo clippy -- -D warnings
    fi
    if [ -f package.json ] || [ -f bun.lockb ]; then
      bun run lint 2>/dev/null || echo "No lint script found in package.json"
    fi
  '';

  scripts.test.exec = ''
    echo "Running tests..."
    if [ -f Cargo.toml ]; then
      cargo test
    fi
    if [ -f package.json ] || [ -f bun.lockb ]; then
      bun test 2>/dev/null || echo "No test script found or no tests configured"
    fi
  '';

  enterShell = ''
    echo
    echo "🚀 Welcome to the Forge development environment!"
    echo
    echo "Available tools:"
    echo "  - Rust ($(rustc --version))"
    echo "  - Bun ($(bun --version))"
    echo
    echo "Available scripts:"
    echo "  - hello    - Say hello"
    echo "  - format   - Format code (Rust + JS/TS)"
    echo "  - lint     - Lint code (Rust + JS/TS)" 
    echo "  - test     - Run tests (Rust + JS/TS)"
    echo
    echo "Get started:"
    echo "  - For Rust: cargo init --name forge-core"
    echo "  - For Bun:  bun init"
    echo
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running devenv tests..."
    hello
  '';

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks = {
    # Rust
    rustfmt.enable = true;
    clippy.enable = true;
    
    # General
    check-yaml.enable = true;
    check-json.enable = true;
    check-toml.enable = true;
    end-of-file-fixer.enable = true;
    trailing-whitespace.enable = true;
    
    # Nix
    nixpkgs-fmt.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}