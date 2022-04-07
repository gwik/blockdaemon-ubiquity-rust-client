# Ubiquity Rust client
A Rust client to the Ubiquity service of blockdaemon.com.

# Requirements
Rust 1.53.0 or newer.

# Installation
Add the following to the `[dependencies]` section of your `Cargo.toml` file:

```
ubiquity = { git = "https://gitlab.com/Blockdaemon/ubiquity/ubiquity-rust-client" }
```

Using a specif branch version, you'll need to replace the placeholder `<branch_name>` (example: `v1`):
```
ubiquity = { git = "https://gitlab.com/Blockdaemon/ubiquity/ubiquity-rust-client", branch = "<branch_name>" }
```

Maybe you need to set a Cargo environment, how it uses git to download the dependencies, check it: https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli

Then, run `cargo build`.

## Testing
To run the test suite just run `cargo test` from the repository's root.

# Usage

## Basic


```rust
use ubiquity::api::blocks_api;
use ubiquity::configuration;
use ubiquity::models::block;


async fn get_block(token: String) -> Option<()>{
    let conf = configuration::Configuration {
        bearer_access_token: Some(token),
        ..configuration::Configuration::new()
    };

    let platform = "ethereum";
    let network = "mainnet";
    let ident = "current";

    let block_result = blocks_api::get_block(&conf, platform, network, ident).await.ok()?;

    let block_id = block_result.id.as_ref()?;
    let block_txs = block_result.txs.as_ref()?;

    Some(println!(
        "Latest block has hash {} and {} transactions",
        block_id,
        block_txs.len()
    ))

}
```

An API URL may also be specified if you have a personal Ubiquity endpoint:

```rust
use ubiquity::api::blocks_api;
use ubiquity::configuration;
use ubiquity::models::block;


async fn get_block(token: String) -> Option<()>{
    let token = "<token>";
    let conf = configuration::Configuration {
        base_path: "<url>".to_string(),
        bearer_access_token: Some(token.to_string()),
        ..configuration::Configuration::new()
    };

    ...
    ...
}
```

## Paginated APIs

Certain resources contain more data than can be practically returned in a single request. In these resources the data is split across multiple responses where each response returns a subset of the items requested and a continuation token.

Requests for the first page of data should not contain a continuation token. To get the next batch of items, the continuation token should be passed with the subsequent request. If no continuation token is returned, all of the available data has been returned.

Initial request to paged APIs should not include a continuation. If no limit is supplied, the default of 25 will be applied.

### Example

(The full example can be found in [`src/lib.rs`](src/lib.rs)'s docs.)

```rust
use transactions_api::GetTxsError;
use ubiquity::api::transactions_api;
use ubiquity::configuration;
use ubiquity::models::TxPage;
use ubiquity::ubiquity_openapi_client::apis::Error;

fn format_get_tx_error(err: Error<GetTxsError>) -> String {
    return format!("{}", err).to_string();
}

...

fn print_tx_page_ids(tx_page: &TxPage) -> Result<(), String> {
  ...
}

async fn print_and_get_continuated_txs(
    conf: &configuration::Configuration,
    platform: &str,
    network: &str,
    order: Option<&str>,
    limit: Option<i32>,
    tx_page: &TxPage,
) -> Result<TxPage, String> {
  ...
}

async fn get_paginated_txs(token: String) -> Result<(), String> {
    let conf = configuration::Configuration {
        bearer_access_token: Some(token),
        ..configuration::Configuration::new()
    };

    let platform = "ethereum";
    let network = "mainnet";

    let order = Some("desc");
    let continuation = None;
    let limit = Some(10);

    // get last 10 transactions
    let tx_result =
        transactions_api::get_txs(&conf, platform, network, order, continuation, limit).await.map_err(format_get_tx_error)?;

    let tx_page = print_and_get_continuated_txs(
        &conf, platform, network, order, limit, &tx_result,
    ).await?;

    print_tx_page_ids(&tx_page)

}
```

## Docs
To generate the crate's documentation, run `cargo doc`.

Additional documentation and examples can be found in the `docs` directory.
