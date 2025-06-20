#!/bin/bash
# Run all Luma JIT-VM examples

echo "=== Luma JIT-VM Examples ==="
echo

for example in examples/*.luma; do
    if [ -f "$example" ]; then
        echo "Running: $example"
        echo "----------------------------------------"
        ./target/release/luma "$example"
        echo
        echo "----------------------------------------"
        echo
    fi
done

echo "All examples completed!"