Simple auth service builded with actix-web and postgres.

# Install and Run
You have to make sure that docker, docker-compose and cargo ^1.56.0 already installed on your system

## Run tests local
Project has some integration tests. Start db before run tests. Also you must use the single thread mod to avoid race condition in database.

run db
```shell
make up_db
```
run tests
```shell
export $(xargs < .env_example) && cargo test -j 1 -- --test-threads=1
```

## Run web service local
```shell
export $(xargs < .env_example) && cargo run
```

# API:
## Basic Auth sign_in `api/v1/users/sign_in`
```shell
curl --location --request POST '127.0.0.1:8080/api/v1/users/sign_in' \
--header 'Authorization: Basic dGVzdF91c2VyOmhlbGxv'
```
Output:
```
{"user_id":2,"jwt_token":"eyJhbGciOiJIUzI1NiJ9.eyJleHBpcmVkX2F0IjoiMjAyMi0wNC0yN1QxMjo1NToxOC41MzE5NzUzNTUrMDA6MDAiLCJncm91cHMiOlsiR1JPVVBfMSIsIkdST1VQXzIiXSwidXNlcl9pZCI6Mn0.PiyObVcPAYS6GbrHAvFWJi9v0JR7ZiQchgOSxSUMyEs"} 
```
Where jwt payload is:
```
{
  "expired_at": "2022-04-27T12:55:18.531975355+00:00",
  "groups": [
    "GROUP_1",
    "GROUP_2"
  ],
  "user_id": 2
}
```
