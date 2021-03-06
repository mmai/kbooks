.DEFAULT_GOAL := build
build:
	cargo +nightly build
install:
	cargo +nightly install
services:
	docker-compose up -d
initdb: services
	diesel setup --config-file diesel-khnum.toml --migration-dir migrations/khnum/postgres/
migrate:
	diesel migration run --config-file diesel-khnum.toml --migration-dir migrations/khnum/postgres/
	diesel migration run --migration-dir migrations/postgres/
# sentry: 
# 	docker-compose -f sentry-docker-compose.yml up 
test:
	# cd kbooks-api && cargo +nightly test --features test
	cd kbooks-api && cargo +nightly-2019-11-29 test --features test
	# cargo test --features test
coverage:
	# launch tests & coverage, for tests only: "cargo test"
	# TODO  add features test
	cargo +nightly tarpaulin -v
run:
	# cargo watch -x run
	# cargo +nightly watch -x run
	# cargo +nightly run
	# cargo +nightly run -p kbooks-api
	cargo +nightly-2019-11-29 run -p kbooks-api
doc:
	cargo +nightly doc --open
cachix:
	nix build | cachix push mmai
