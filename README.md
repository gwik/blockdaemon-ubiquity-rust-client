# ubiquity-rust-client
A Rust client to the Ubiquity service of blockdaemon.com.

# Requirements
Rust 1.53.0 or newer.

# Installation
Add the following to the `[dependencies]` section of your `Cargo.toml` file:

```
ubiquity = { git = "https://gitlab.com/Blockdaemon/ubiquity/ubiquity-rust-client" }
```

Then, run `cargo build`.

## Testing
To run the test suite just run `cargo test` from the repository's root.

# Usage

## Basic

(Full example can be found at [`examples/get-block`](examples/get-block)).

```rust
use ubiquity::api::blocks_api;
use ubiquity::configuration;
use ubiquity::models::block;

fn print_block_result(block: &block::Block) -> Option<()> {
    let block_id = block.id.as_ref()?;
    let block_txs = block.txs.as_ref()?;

    return Some(println!(
        "Latest block has hash {} and {} transactions",
        block_id,
        block_txs.len()
    ));
}

#[tokio::main]
async fn main() {
    let token = "<token>";
    let conf = configuration::Configuration {
        bearer_access_token: Some(token.to_string()),
        ..configuration::Configuration::new()
    };

    let platform = "ethereum";
    let network = "mainnet";
    let ident = "current";

    let block_result = blocks_api::get_block(&conf, platform, network, ident).await;

    match block_result {
        Ok(block) => print_block_result(&block),
        Err(e) => panic!("{}", e),
    };
}
```

An API URL may also be specified if you have a personal ubiquity endpoint 
```rust
use ubiquity::api::blocks_api;
use ubiquity::configuration;
use ubiquity::models::block;

...
...

#[tokio::main]
async fn main() {
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

## Paginated API's

Certain resources contain more data than can practically returned in a single request. In these resources the data is split across multiple responses where each response returns a subset of the items requested and a continuation token. Requests for the first page of data should not contain a continuation token. To get the next batch of items the continuation token should be passed with the subsequent request. If no continuation token is returned all of the available data has been returned.

Initial request to paged API's should not include a continuation. If no limit is supplied the default of 25 will be applied.

### Example

(Full example can be found at [`examples/pagination`](examples/pagination)).

```rust
use transactions_api::GetTxsError;
use ubiquity::api::transactions_api;
use ubiquity::configuration;
use ubiquity::models::TxPage;
use ubiquity::ubiquity_openapi_client::apis::Error;

fn format_get_tx_error(err: Error<GetTxsError>) -> String {
    return format!("{}", err).to_string();
}

async fn get_continuated_txs(
    conf: &configuration::Configuration,
    platform: &str,
    network: &str,
    order: Option<&str>,
    limit: Option<i32>,
    assets: Option<&str>,
    tx_page: &TxPage,
) -> Result<TxPage, String> {
  ...
}

fn print_tx_page_ids(tx_page: &TxPage) -> Result<(), String> {
  ...
}

async fn print_and_get_continuated_txs(
    conf: &configuration::Configuration,
    platform: &str,
    network: &str,
    order: Option<&str>,
    limit: Option<i32>,
    assets: Option<&str>,
    tx_page: &TxPage,
) -> Result<TxPage, String> {
  ...
}

#[tokio::main]
async fn main() {
    let token = "<token>";
    let conf = configuration::Configuration {
        bearer_access_token: Some(token.to_string()),
        ..configuration::Configuration::new()
    };

    let platform = "ethereum";
    let network = "mainnet";

    let order = Some("desc");
    let continuation = None;
    let limit = Some(10);
    let assets = None;

    // get last 10 transactions
    let tx_result =
        transactions_api::get_txs(&conf, platform, network, order, continuation, limit, assets)
            .await;

    // To continue through the pages of transactions the continuation
    //  from the previous page must be supplied to the next request:
    match tx_result {
        Ok(tx_page) => {
            // gets next 10 transactions from continuation value
            match print_and_get_continuated_txs(
                &conf, platform, network, order, limit, assets, &tx_page,
            )
            .await
            .and_then(|tx_page| print_tx_page_ids(&tx_page))
            {
                Ok(_tx_page) => {}
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    };
```

## Docs
Additional documentation and examples can be found in the `docs` directory.
