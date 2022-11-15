# Pipedrive GateWay
Gateway so you can easily and safely add pipedrive lead generation forms to your website

## How to build

```shell
cargo build
cargo build --release
```

You can find the binary in `target/debug/` or `target/release/` folder

## How to run
```shell
cargo run --bin user
```

## Process
```shell
curl http://127.0.0.1:7777/AddCrmLead -d '
{
    "email": "123@gmail.com",         
    "username": "pepe",            
    "title": "test",            
    "message": "test content"
}'

{}

```

## Structure explained

`src/codegen` core codegen logic

`src/gen` codegen target

`src/lib` common code

`src/service` implementation of services

`src/service/{srv}/main.rs` main entry of service

`src/service/{srv}/endpoints.rs` declaration of endpoints

`src/service/{srv}/pg_func.rs` declaration of postgres procedural endpoints (DALs)

`src/service/{srv}/method.rs` implementation of endpoints

`tests` integration tests

`benches` benchmarks

`docs` documentation

`db` database related files

`etc` configuration files

`scripts` helper scripts
