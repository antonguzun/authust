up_db:
	docker-compose up -d
	sleep 3
	psql postgres://postgres:dbpass@0.0.0.0:5432/db psql -c \
		"BEGIN TRANSACTION;" \
		-f tests/migrations/V1__add_users.sql \
		-f tests/migrations/V2__add_role_perms.sql \
		-f tests/migrations/V3__add_roles_members.sql -c "COMMIT;"

down_db:
	docker-compose down
