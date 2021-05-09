SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: help lint test

help: ## List all available targets with help
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

test:
	cargo test

lint:
	cargo clippy

docker-build:
	@docker build -t gcd-web-server-image .

compose-up:
	@docker compose -p gcd-web-server up -d

run-docker: docker-build compose-up

