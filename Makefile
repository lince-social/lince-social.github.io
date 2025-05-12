.PHONY: dev

dev:
	@MDBOOK_BIN=$$(which mdbook 2>/dev/null || echo "$$HOME/.cargo/bin/mdbook"); \
	if [ ! -x "$$MDBOOK_BIN" ]; then \
		echo "mdbook not found. Installing..."; \
		cargo install mdbook; \
	fi; \
	echo "Using mdbook at $$MDBOOK_BIN"; \
	"$$MDBOOK_BIN" serve --port 9999

build:
	mdbook build;
	rm -rf css FontAwesome fonts
	mv book/* ./
	git add .
	git commit
	git push
