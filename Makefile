build_and_move_cli:
	cargo b --bin noops && sudo mv target/debug/noops /usr/local/bin

run_server:
	cargo run --bin noops-server