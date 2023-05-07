# rest_api

## Migrating

```console
sqlx migrate run --database-url "postgresql://admin:password@localhost:5432/db?schema=public"

# revert
sqlx migrate revert --database-url "postgresql://admin:password@localhost:5432/db?schema=public"
```

## Docker

```console
docker-compose up -d

docker-compose down

# get psql shell
docker exec -it postgres psql db admin
```
