#!/bin/bash

# Set the output directory for the coverage report
OUTPUT_DIR="."

# Remove old coverage report directory
if [ -d "$OUTPUT_DIR/html" ]; then
    echo "Removing old coverage report directory..."
    rm -rf "$OUTPUT_DIR/html"
fi

# Clean the project
echo "Cleaning the project..."
cargo clean

# Build the project with coverage instrumentation
echo "Building the project with coverage instrumentation..."
cargo build --profile dev --verbose

# Run tests with coverage
echo "Running tests with coverage instrumentation..."
cargo llvm-cov test --branch --verbose

# Generate the HTML coverage report
echo "Generating HTML coverage report..."
cargo llvm-cov --html --branch --output-dir="$OUTPUT_DIR"

echo "HTML coverage report generated in the '$OUTPUT_DIR' directory."
