export DB_HOST = localhost
export DB_PORT = 6543
export DB_USERNAME = admin
export DB_PASSWORD = password
export DB_NAME = poc

DOCKER_COMPOSE ?= docker compose

.PHONY: bench clean docker_compose_down docker_compose_up

docker_compose_up:
	$(DOCKER_COMPOSE) up --detach

bench: docker_compose_up
	@cargo bench

docker_compose_down:
	$(DOCKER_COMPOSE) down --remove-orphans

clean: docker_compose_down
	@cargo clean
