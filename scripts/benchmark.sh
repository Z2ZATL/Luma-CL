#!/bin/bash
# Benchmark Luma JIT-VM performance

echo "=== Luma JIT-VM Performance Benchmark ==="
echo

# Build optimized version
echo "Building optimized version..."
cargo build --release
echo

# Create benchmark program
cat > benchmark_temp.luma << 'EOF'
# Fibonacci benchmark
let n be 30
let a be 0
let b be 1
let i be 2

while i <= n then
    let temp be a + b
    a = b
    b = temp
    i = i + 1

show "Fibonacci(" + n + ") = " + b

# Mathematical computation benchmark
let iterations be 10000
let result be 0
let j be 0

while j < iterations then
    result = result + (j * j) / (j + 1) - (j % 7)
    j = j + 1

show "Computation result: " + result
EOF

echo "Running benchmark..."
echo "----------------------------------------"
time ./target/release/luma benchmark_temp.luma
echo "----------------------------------------"

# Cleanup
rm -f benchmark_temp.luma

echo "Benchmark completed!"