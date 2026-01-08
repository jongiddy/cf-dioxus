#!/bin/bash
# Clean cargo build artifacts in all crates with Cargo.toml files

set -e

echo "Finding all Cargo.toml files and running cargo clean..."

# Find all Cargo.toml files and get their directories
while IFS= read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    echo "Cleaning: $dir"
    (cd "$dir" && cargo clean)
done < <(find . -name "Cargo.toml" -type f | sort -V)

echo "Done! All cargo build artifacts cleaned."
