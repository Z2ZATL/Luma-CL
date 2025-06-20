#!/bin/bash
# Run all Luma JIT-VM tests

echo "=== Luma JIT-VM Test Suite ==="
echo

# Build first
echo "Building Luma JIT-VM..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi
echo "Build successful!"
echo

# Run tests
for test in tests/*.luma; do
    if [ -f "$test" ]; then
        echo "Running test: $test"
        echo "----------------------------------------"
        time ./target/release/luma "$test"
        if [ $? -eq 0 ]; then
            echo "✓ PASS: $test"
        else
            echo "✗ FAIL: $test"
        fi
        echo
        echo "----------------------------------------"
        echo
    fi
done

echo "Test suite completed!"