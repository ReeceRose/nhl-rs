# NHL-RS

The unofficial NHL Rust SDK for the new Edge API.

## Supported Stats Endpoints
- /country
- /franchise
- /glossary
- /ping
- /game/meta

## Supported Edge Endpoints

More coming soon!

## How to Use

1. Add the crate to your project: `cargo add nhl-rs`
2. Import the NHL-RS Client `use nhl_rs::Client;`
3. Create a new client: `let client = ClientBuilder::new().build();`
4. Query the NHL API:
```rust
let response = client.get_franchise_by_id(1).await?;
```

Full example:

```rust
use nhl_rs::Client;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new().build();

    let response = client.get_franchise_by_id(1).await?.unwrap();

    println!("Confrence with the ID of 1: {:?}", response);
}
```

## Run Examples

The examples folder is full of examples for all the currently [supported endpoints](#Supported-Endpoints). To run the conferences example, execute:
```bash
cd examples
# cargo run --example franchises
```

## Errors

You can expect all calls to return a Result with the Ok variant being the appropriate data and the Err variant being a u16 which matches the error code received from the NHL API (Only in the error case, i.e. no 200 status codes).
