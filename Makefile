build:
	cargo fmt --all && cargo clippy --all --tests

start_postgres_dev:
	sudo docker compose -f dockers/postgres_only/docker-compose.yaml up

test_db:
	cargo run -- db 