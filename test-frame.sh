#!/bin/bash
# Quick test script

echo "Test 1: Direct process() call with frame"
cargo run --release --example test-direct 2>&1 | tail -3

echo ""
echo "Test 2: Via stdin"
echo "{{frame:gradient}}test{{/frame}}" | cargo run --release -- process 2>&1 | tail -3

echo ""
echo "Test 3: Via file"
echo "{{frame:gradient}}test{{/frame}}" > /tmp/frame-test.md
cargo run --release -- process /tmp/frame-test.md 2>&1 | tail -3
