<!-- TOC -->

- [rust-app](#rust-app)
    - [instructions](#instructions)
        - [step 1 - setup tokio](#step-1---setup-tokio)
        - [step 2 - setup axum](#step-2---setup-axum)
        - [step 3 - implement todos_get api](#step-3---implement-todos_get-api)

<!-- /TOC -->

# rust-app

Demo rust application

## instructions

### step 1 - setup tokio

```shell
# initialise a new app
caro init
# add tokio
cargo add tokio -F full
# update main.rs
# run app, expect Hello, world! printed in the console
cargo run
```

### step 2 - setup axum

```shell
cargo add axum
cargo add --dev tower -F util
cargo add --dev http-body-util
# add the routes function in main.rs and add the tests
# run test, it should pass
cargo test
# run the server, the server should start listening on port 3000
cargo run
# in a separate terminal use curl, should return Hello, World!!
curl http://localhost:3000/plaintext
```

### step 3 - implement todos_get api

```shell
cargo add serde -F derive
cargo add --dev serde_json
# create ToDo struct and Db using ToDo
# implement todos_get
# create dummy todos and add it to db in routes()
# add get /todos route using todos_get
# implement test_todos_get  test
# run test, it should pass 2 tests
cargo test
# run the server, the server should start listening on port 3000
cargo run
# in a separate terminal use curl, should return 2 ToDos. return content-type: application/json
curl -vs  http://localhost:3000/todos | jq
```
