add_test_env:
	cp .env_example .env

up_db:
	docker-compose up -d
	sleep 3
	psql postgres://postgres:dbpass@0.0.0.0:5432/db -f init.sql

down_db:
	docker-compose down

test:
	export $(xargs < .env_example) && cargo test -j 1 -- --test-threads=1
