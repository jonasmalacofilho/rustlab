#!/bin/bash

for impl in for-loops for_each while-loops;
do
	echo "* IMPLEMNATION: $impl"

	echo "- Running tests..."
	cargo test --no-default-features --features $impl

	echo "- Executing in debug mode (with debug assertions)..."
	cargo run --no-default-features --features $impl

	echo "- Executing..."
	cargo run --release --no-default-features --features $impl
done
