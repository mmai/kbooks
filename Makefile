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
	cargo +nightly-2019-11-29 test
	# cargo test
coverage:
	# launch tests & coverage, for tests only: "cargo test"
	cargo +nightly tarpaulin -v
run:
	# cargo watch -x run
	# cargo +nightly watch -x run
	# cargo +nightly run
	cargo +nightly-2019-11-29 run
doc:
	cargo +nightly doc --open
