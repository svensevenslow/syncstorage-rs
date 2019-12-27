##
# Collection of helper scripts used for local dev.
##

DOCKER_COMPOSE_FILE = docker-compose.yaml

run:
	RUST_LOG=debug RUST_BACKTRACE=full cargo run -- --config config/dev.toml

start_docker:
	docker-compose -f $(DOCKER_COMPOSE_FILE) up -d

start_docker_rebuild:
	docker-compose -f $(DOCKER_COMPOSE_FILE) up --build -d

stop_docker:
	docker-compose -f $(DOCKER_COMPOSE_FILE) down
