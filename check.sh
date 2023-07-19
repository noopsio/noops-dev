#!/bin/bash -e


if [[ -z "$1" || "$1" == "help" ]]; then
    script_name="$(basename "$0")"
    echo -e "Checks the code like the CI\n"
    echo -e "Usage: ${script_name} <scope>\n"
    echo "Scope:"
    echo -e "\tserver"
    echo -e "\tcli"
    exit 0
fi    

if [[ "$1" == "server" ]]; then
    echo "[Server] Linting checks..."
    cargo clippy -p noops-server -- -D warnings
    cargo clippy --tests -p noops-server -- -D warnings

    echo "[Server] Build..."
    cargo build -p noops-server

    echo "[Server] Test..."
    cargo test -p noops-server

elif [[ "$1" == "cli" ]]; then
    echo "[CLI] Linting checks..."
    cargo clippy -p noops-server -- -D warnings
    cargo clippy --tests -p noops -- -D warnings

    echo "[CLI] Build..."
    cargo build -p noops

    echo "[CLI] Test..."
    cargo test -p noops
else
    echo "Scope \""$1"\" not found"
    exit 1
fi
    