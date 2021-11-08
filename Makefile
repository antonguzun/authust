up_db:
	docker-compose up -d
	sleep 3
	psql postgres://postgres:dbpass@0.0.0.0:5432/db -f init.sql

down_db:
	docker-compose down