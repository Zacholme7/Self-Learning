#!/bin/bash

# Function to recursively search for Cargo.toml files and run cargo clean
clean_cargo_projects() {
    for dir in "$1"/*; do
        if [ -d "$dir" ]; then
            if [ -f "$dir/Cargo.toml" ]; then
                echo "Cleaning Rust project in $dir"
                (cd "$dir" && cargo clean)
            fi
            clean_cargo_projects "$dir"
        fi
    done
}

# Start the recursive search from the current directory
clean_cargo_projects "$(pwd)"

echo "Finished cleaning all Rust projects."
