#!/bin/sh
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

frpc -c ./frpc.toml &
cargo r

