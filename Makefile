# Helper scripts used for local dev.
run:
	RUST_LOG=debug RUST_BACKTRACE=full cargo run -- --config config/dev.toml

start_docker:
	docker-compose -f docker-compose.dev.yaml up -d

stop_docker:
	docker-compose -f docker-compose.dev.yaml down
