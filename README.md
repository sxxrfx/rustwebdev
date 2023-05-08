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

## Curl Commands

### Questions

### GET

```console
curl --location 'localhost:3030/questions?limit=10&offset=0'
```

### DELETE

```console
curl --location --request DELETE 'localhost:3030/questions/1'
```

### PUT

```console
curl --location --request PUT 'localhost:3030/questions/1' \
--header 'Content-Type: application/json' \
--data '{
    "id": 1,
    "title": "New title",
    "content": "Content",
    "tags" : []
}'
```

### POST

```console
curl --location 'localhost:3030/questions' \
--header 'Content-Type: application/json' \
--data '{
    "title": "First Question",
    "content": "What is your name?",
    "tags": ["intro"]
}'
```

## Answer

### POST answer

```console
curl --location 'localhost:3030/answers' \
--header 'Content-Type: application/json' \
--data '{
    "question_id": 4,
    "content": "This is answer of fourth question."
}'
```
