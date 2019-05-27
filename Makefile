services:
	docker-compose up -d
initdb: services
	diesel setup
# sentry: 
# 	docker-compose -f sentry-docker-compose.yml up 
test:
	# launch tests & coverage, for tests only: "cargo test"
	cargo tarpaulin -v
run:
	cargo watch -x run
doc:
	cargo doc --open
