#!/bin/sh
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

frpc &
cargo r

