#!/bin/sh
trap "trap - TERM && kill -- -$$" INT TERM EXIT
set -a; . ./.env; set +a; # Exporting .env

frpc -c ./frpc.toml &

cargo r

