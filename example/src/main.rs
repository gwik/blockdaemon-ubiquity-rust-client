use ubiquity::api::{balances_utxo_api, blocks_api, transactions_api};
use ubiquity::configuration;

#[tokio::main]
async fn main() {
    println!("Example started");

    let token = "<add token here>".to_string();
    let protocol = "litecoin";
    let network = "mainnet";
    let addr = "LLucub4nwTdmSd2dhi4fnPLj9vaV6V2Bfw";

    let cg = configuration::Configuration {
        // bearer_access_token: Some(token), // Using Bearer token, both works `api_key` and `bearer`
        api_key: Some(configuration::ApiKey{
            prefix: None,
            key: token.to_string(),
        }),
        ..configuration::Configuration::new()
    };

    let result = blocks_api::get_current_block_hash(&cg, protocol, network).await;
    match result {
        Ok(id) => println!("current block: {}", id),
        Err(e) => panic!("{}", e),
    }

    let balance_map =
        balances_utxo_api::get_list_of_balances_by_address(&cg, protocol, network, addr, None).await;

    match balance_map {
        Ok(b) => {
            let f = b.first().unwrap();
            println!("Balance: {:?}", f);

            Some(println!("ok"));
        }
        Err(e) => panic!("{}", e),
    }

    let utxos_result = transactions_api::get_utxoby_account(&cg, protocol, network, addr, Some(false), Some(false), None, None, None, None, Some(10)).await;
    match utxos_result {
        Ok(utxos) => {
            println!("Total UTXO: {:?} - Continuation: {:?}", utxos.total.unwrap(), utxos.meta);
            for u in utxos.data.unwrap_or_else(Vec::new).iter() {
                println!("{:?}", u);
            }

            Some(println!("ok"));
        }
        Err(e) => panic!("{}", e),
    }
}
