# Rust API client for The Blue Alliance

## Overview 

 Information and statistics about FIRST Robotics Competition teams and events.
 
## Quickstart

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```toml
[dependencies]
    tba-v3 = "0.1.0"
```

```rust
#[tokio::test]
async fn team_works() {
    // Create a client
    let api_key = "API_KEY".to_owned();
    let config = Rc::new(Configuration::from_api_key(api_key));
    let api = TeamApiClient::new(config.clone());

    // Use the client
    let result = api.get_team_event_matches("frc254", "2019cmptx", None).await.unwrap();
    println!("{:?}", result);
}
```

## Authentication 

All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account). 

 A `User-Agent` header may need to be set to prevent a 403 Unauthorized error.

## Documentation for API Endpoints

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------


## Documentation For Models



To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author
Wes Jordan