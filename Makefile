# Makefile for aurelis workspace
# Orquestra build, OpenAPI generation and TypeScript client codegen.
#
# Usage:
#   make help            # show help
#   make build           # build all workspace crates (debug)
#   make build-release   # build all workspace crates (release)
#   make gen-openapi     # build+run gen_openapi and write openapi.json into admin panel
#   make gen-client      # run the TS client generator (prefers pnpm script, falls back to npx)
#   make codegen-all     # gen-openapi then gen-client
#   make fmt             # run rustfmt
#   make test            # run cargo tests
#   make clean           # remove generated OpenAPI and generated client
#
# Edit variables below to match your preferences.

SHELL := /bin/sh

# Tools (can be overridden in environment)
CARGO ?= cargo
PNPM  ?= pnpm
NPX   ?= npx

# Project-specific settings
ADMIN_DIR    ?= crates/admin-panel
CRATE        ?= api-server
GEN_BIN      ?= gen_openapi
OPENAPI_JSON ?= $(ADMIN_DIR)/openapi.json
CLIENT_OUT   ?= $(ADMIN_DIR)/src/api

.PHONY: help build build-release fmt test clean gen-openapi gen-client codegen-all openapi codegen

help:
	@cat <<'EOF'
Make targets:
  help           show this message
  build          cargo build --workspace (debug)
  build-release  cargo build --workspace --release
  fmt            cargo fmt --all
  test           cargo test --workspace
  gen-openapi    run the Rust helper to emit OpenAPI JSON to $(OPENAPI_JSON)
  gen-client     generate TypeScript client into $(CLIENT_OUT) (uses pnpm script if available)
  codegen-all    gen-openapi && gen-client
  clean          remove generated OpenAPI JSON and generated client
EOF

# Basic build helpers
build:
	$(CARGO) build --workspace

build-release:
	$(CARGO) build --workspace --release

fmt:
	$(CARGO) fmt --all

test:
	$(CARGO) test --workspace

# Generate OpenAPI JSON by running the helper binary (gen_openapi).
# This will build the binary if necessary.
gen-openapi:
	@echo "Generating OpenAPI JSON -> $(OPENAPI_JSON)"
	@mkdir -p $(dir $(OPENAPI_JSON))
	@$(CARGO) run -p $(CRATE) --bin $(GEN_BIN) --release > $(OPENAPI_JSON)

# Generate TypeScript client.
# Prefer invoking the admin panel's npm script (pnpm run codegen).
# If pnpm isn't found, fall back to npx openapi-typescript-codegen directly.
gen-client:
	@if [ ! -f "$(OPENAPI_JSON)" ]; then \
	  echo "ERROR: OpenAPI file '$(OPENAPI_JSON)' not found. Run 'make gen-openapi' first."; \
	  exit 1; \
	fi
	@if command -v $(PNPM) >/dev/null 2>&1; then \
	  echo "Running '$(PNPM) run codegen' in $(ADMIN_DIR)"; \
	  cd $(ADMIN_DIR) && $(PNPM) run codegen; \
	else \
	  echo "pnpm not found - falling back to npx openapi-typescript-codegen"; \
	  mkdir -p $(CLIENT_OUT); \
	  $(NPX) openapi-typescript-codegen --input $(OPENAPI_JSON) --output $(CLIENT_OUT) --client fetch --useOptions --name ApiClient; \
	fi

# Convenience aliases
openapi: gen-openapi
codegen: gen-client

codegen-all: gen-openapi gen-client

# Remove generated artifacts (be conservative - only generated openapi.json and client dir)
clean:
	@echo "Cleaning generated OpenAPI and client"
	@rm -f $(OPENAPI_JSON)
	@rm -rf $(CLIENT_OUT)

# End of Makefile
