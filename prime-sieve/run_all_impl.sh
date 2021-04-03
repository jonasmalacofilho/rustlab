#!/bin/bash

for stor in bool-based bit-based;
do
	for impl in for-loops for_each while-loops;
	do
		echo "****************************************"
		echo "* STORAGE: $stor, IMPLEMENTATION: $impl"

		echo "- Running tests..."
		cargo test --no-default-features --features $stor,$impl

		echo "- Executing in debug mode (with debug assertions)..."
		cargo run --no-default-features --features $stor,$impl

		echo "- Executing..."
		cargo run --release --no-default-features --features $stor,$impl
	done
done
