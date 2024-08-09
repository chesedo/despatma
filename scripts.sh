#!/usr/bin/env sh

# Run tests
ct() {
    echo "Running tests..."
    cargo test --all-features
}

# Run clippy
cc() {
    echo "Running Clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
}

# Check formatting
cf() {
    echo "Checking code formatting..."
    cargo fmt -- --check
}

# Allow calling functions from command line
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    "$@"
else
    printf "\n\033[1;33m• Available Functions:\033[0m"
    printf "\n  - ct : Run tests"
    printf "\n  - cc : Run clippy"
    printf "\n  - cf : Check formatting"
fi
