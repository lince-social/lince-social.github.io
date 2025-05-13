.PHONY: dev

dev:
	@MDBOOK_BIN=$$(which mdbook 2>/dev/null || echo "$$HOME/.cargo/bin/mdbook"); \
	if [ ! -x "$$MDBOOK_BIN" ]; then \
		echo "mdbook not found. Installing..."; \
		cargo install mdbook; \
	fi; \
	echo "Using mdbook at $$MDBOOK_BIN"; \
	"$$MDBOOK_BIN" serve --port 6173

build:
	mdbook build;
	git add .
	git commit
	git push
