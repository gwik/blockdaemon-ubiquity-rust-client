# Paginating over the network's latest transactions
# Example

This shows how to paginate over the latest transactions from the network.

It gets the 10 latest transactions from Ethereum's mainnet, then paginate through the 10 next ones, printing their IDs.

## Running
The environment variable `ACCESS_TOKEN` need to be set with an Ubiquity access token.
Then, just run `cargo run` from this directory. For example:

```bash
$ ACCESS_TOKEN=<token> cargo run
```
