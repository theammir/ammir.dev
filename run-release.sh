#!/bin/sh
trap "trap - TERM && kill -- -$$" INT TERM EXIT

set -a; . ./.env; set +a; # Exporting .env

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

"$BINARY_PATH" 
