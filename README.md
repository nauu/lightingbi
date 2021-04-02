# lightingbi
Intelligent analysis system by Rust, A Practice Rust project

# Usage

## Prerequisites

* Rust
* MySQL


## Set up the database

* Create new database using `components/user/schema.sql`

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