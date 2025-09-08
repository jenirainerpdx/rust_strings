help:
	@grep -E '^[a-zA-Z_-]+:' Makefile | sed 's/:.*#/#/' | column -t -s '#'

clean: ## Clean the project using cargo. Also runs format.
	cargo clean

build: ## Build the project using cargo
	cargo build

test: ## Run tests using cargo
	cargo test

fmt: ## Format the code
	cargo fmt --quiet

lint: ## Check code style and check for errors and warnings. Also checks formatting.
	cargo clippy --quiet

doc: ## Generate documentation
	cargo doc

make run: ## Run the project
	cargo run
	