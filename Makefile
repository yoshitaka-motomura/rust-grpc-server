.PHONY: build serve dev help 

build: ## build the project
	@echo "Building the project..."
	@docker buildx build --platform linux/arm64 -t rust-grpc-server:latest .
serve: ## serve the project
	@docker compose up -d

dev: ## develop
	@RUST_LOG=debug DESCRIPTOR_FILE_PATH=proto/descriptor.bin cargo run
help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
