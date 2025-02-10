# Easee API Client

This is a Rust client implementation for the [Easee API](https://developer.easee.com/). It provides an interface for interacting with Easee's EV charging platform, enabling users to authenticate, retrieve charger status, and control charging sessions.

## Features

- Authentication with Easee API
- Fetching charger details and status
- Starting and stopping charging sessions
- Configuring charger settings

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
easee-rs = "0.1"
```

(Note: Replace with the actual crate name if published on crates.io, or use a Git dependency if needed.)

## Usage

```rust
use easee_rs::Client;

#[tokio::main]
async fn main() {
    let client = Client::new_with_access_token("your-api-key");
    match client.get_chargers().await {
        Ok(chargers) => println!("{:?}", chargers),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Authentication

The client requires either an API key, or username and password for authentication. The API key can be obtained from 
the Easee developer portal, while the username and password are the same as those used to log in to the Easee app.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>


