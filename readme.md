Simple crud web service with actix-web and async postgres integration.

### Install and Run
You have to make sure that docker, docker-compose and cargo ^1.56.0 already installed on your system

#### How to run dependencies
```shell
make up_db
```

#### Run web service
```shell
export $(xargs < .env_example) && cargo run
```

### API:

#### insert item
```shell
curl --location --request POST '127.0.0.1:8080/api/v1/user' \
--header 'Content-Type: application/json' \
--data-raw '{"name": "Jeff"}'
```
`{"id":4,"name":"Jeff"}`

#### get item
```shell
curl --location --request GET '127.0.0.1:8080/api/v1/user/4'
```
`{"id":4,"name":"Jeff"}`

#### get listing
```shell
curl --location --request GET '127.0.0.1:8080/api/v1/user?limit=4'
```
`
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
`
#### delete item
```shell
curl --location --request DELETE '127.0.0.1:8080/api/v1/user/4' -i '
```
`HTTP/1.1 204 No Content`


### Tests
In progress