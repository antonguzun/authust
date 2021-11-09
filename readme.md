This is simple crud web service with postgres integration.

### Install and Run
You have to make sure that docker, docker-compose and cargo ^1.56.0 already installed on your system

How to run dependencies:
```shell
make up_db
```

Run web service
```shell
make run_dev
```

### API:

insert item
```shell
curl --location --request POST '127.0.0.1:8080/api/v1/entity' \                                                                                        andyguzun@andyguzun-mac
--header 'Content-Type: application/json' \
--data-raw '{"name": "Jeff"}'
```
`{"id":4,"name":"Jeff"}`

get item
```shell
curl --location --request GET '127.0.0.1:8080/api/v1/entity/4'                                                                                     1 ↵ andyguzun@andyguzun-mac
```
`{"id":4,"name":"Jeff"}`

get listing
```shell
curl --location --request GET '127.0.0.1:8080/api/v1/entity?limit=4'
```
```
[
    {
        "id": 1,
        "name": "Ivan"
    },
    {
        "id": 2,
        "name": "Anton"
    },
    {
        "id": 3,
        "name": "Godzilla"
    },
    {
        "id": 4,
        "name": "Jeff"
    }
]
```

### Tests
In progress