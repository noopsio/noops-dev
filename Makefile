build_and_move_cli:
	cargo b --bin noops && sudo mv target/debug/noops /usr/local/bin

run_server:
	RUST_LOG=DEBUG cargo run --bin noops-server