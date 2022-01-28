.PHONY: build
build:
	cargo build

.PHONY: build-web
build-web:
	yarn build:release

.PHONY: install
install: build-web
	cargo install --force --path .

.PHONY: time
time:
	cargo +nightly build -Z timings

.PHONY: commit
commit:
	pre-commit run --all-files
