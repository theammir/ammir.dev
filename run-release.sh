#!/bin/sh
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
frpc -c ./frpc.toml &

BINARY_PATH="./target/release/ammir-dev"

if [ -f "$BINARY_PATH" ]; then
    if cargo build --release --quiet --color always; then
        echo "Binary is up to date."
    else
        echo "Rebuilding the binary..."
    fi
else
    echo "Binary not found. Building..."
    cargo build --release
fi

# Run the binary
"$BINARY_PATH"
