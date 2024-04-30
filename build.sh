#!/bin/bash

# Script to build and optimize a Rust program for publishing on Linux and Windows using Cargo

# Function to display usage information
usage() {
    echo "Usage: $0 <project_directory>"
    echo "Example: $0 my_project"
    exit 1
}

# Check if project directory argument is provided
if [ $# -ne 1 ]; then
    usage
fi

# Project directory
PROJECT_DIR=$1
rm -rf ./bin
# Check if the project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
    echo "Error: Project directory '$PROJECT_DIR' not found."
    exit 1
fi

# Change directory to project directory
cd "$PROJECT_DIR" || exit

# Build the project for Linux using musl target for static linking
echo "Building project for Linux..."
cargo build --release --target=x86_64-unknown-linux-musl
if [ $? -eq 0 ]; then
    echo "Build for Linux successful."
    mkdir -p bin/linux
    mv target/x86_64-unknown-linux-musl/release/* bin/linux/
    echo "Executables moved to bin/linux directory."
else
    echo "Build for Linux failed."
fi

# Build the project for Windows
echo "Building project for Windows..."
cargo build --release --target=x86_64-pc-windows-gnu
if [ $? -eq 0 ]; then
    echo "Build for Windows successful."
    mkdir -p bin/windows
    mv target/x86_64-pc-windows-gnu/release/* bin/windows/
    echo "Executables moved to bin/windows directory."
else
    echo "Build for Windows failed."
fi
