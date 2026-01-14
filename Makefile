DB_IMAGE := postgres:18-alpine
DB_CONTAINER_NAME := segmento-referral-postgres
DB_NAME := segmento
DB_USER_NAME := postgres
DB_PASSWORD := postgres
DB_CRATE_PATH := crates/database

db:
	docker run --name $(DB_CONTAINER_NAME) -p 5432:5432 -e POSTGRES_USER=$(DB_USER_NAME) -e POSTGRES_PASSWORD=$(DB_PASSWORD) -u $(DB_USER_NAME) -d $(DB_IMAGE)

createdb:
	docker exec -it $(DB_CONTAINER_NAME) createdb --user=$(DB_USER_NAME) --owner=$(DB_USER_NAME) $(DB_NAME)

dropdb:
	docker exec -it $(DB_CONTAINER_NAME) dropdb $(DB_NAME)

psql:
	docker exec -it -u postgres $(DB_CONTAINER_NAME) psql -d $(DB_NAME)

migrateup:
	cd ${DB_CRATE_PATH} && diesel migration run

migratedown:
	cd ${DB_CRATE_PATH} && diesel migration revert

migrateredo:
	cd ${DB_CRATE_PATH} && diesel migration redo

generateschema:
	cd ${DB_CRATE_PATH} && diesel print-schema > src/schema.rs

.PHONY: db createdb dropdb migrateup migratedown migrateredo generateschema
