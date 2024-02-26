DOCKER_COMPOSE ?= docker compose

.PHONY: clean docker_compose_down docker_compose_up

docker_compose_up:
	$(DOCKER_COMPOSE) up --detach

docker_compose_down:
	$(DOCKER_COMPOSE) down --remove-orphans

clean: docker_compose_down
	@cargo clean
