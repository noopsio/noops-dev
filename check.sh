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

function check_server() {
    echo "[Server] Linting checks..."
    cargo clippy -p noops-server -- -D warnings
    cargo clippy --tests -p noops-server -- -D warnings

    echo "[Server] Build..."
    cargo build -p noops-server

    echo "[Server] Test..."
    cargo test -p noops-server
}

function check_cli() {
    echo "[Server] Linting checks..."
    cargo clippy -p noops-server -- -D warnings
    cargo clippy --tests -p noops-server -- -D warnings

    echo "[Server] Build..."
    cargo build -p noops-server

    echo "[Server] Test..."
    cargo test -p noops-server
}

if [[ "$1" == "server" ]]; then
    check_server
elif [[ "$1" == "cli" ]]; then
    check_cli
elif [[ "$1" == "all" ]]; then
    check_server
    check_cli
else
    echo "Scope \""$1"\" not found"
    exit 1
fi
    