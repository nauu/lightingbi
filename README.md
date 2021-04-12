# lightingbi
Intelligent analysis system by Rust, A Practice Rust project.

# Usage

## Prerequisites

* Rust
* MySQL


## Set up the database

* Create new database using `src/schema.sql`

## Run the application

```bash
cargo run --bin lightingbi
```

## Endpoints

    GET http://127.0.0.1:5002/playground      GraphQL Playground UI

## Query Examples

```graphql
{
  users 
}
```

# Build

```shell
cargo install cross

#x86_64 linux
cross build --target=x86_64-unknown-linux-musl --release --features vendored
```

```shell
#docker
docker build -t lightningbi:0.1.0 .
```

```shell
#doc
cargo doc --no-deps --workspace  --open

```