use ubiquity::api::accounts_api;
use ubiquity::api::sync_api;
use ubiquity::configuration;

#[tokio::main]
async fn main() {
    println!("Example started");

    let token = "<add token here>".to_string();
    let platform = "litecoin";
    let network = "mainnet";
    let addr = "LLucub4nwTdmSd2dhi4fnPLj9vaV6V2Bfw";

    let cg = configuration::Configuration {
        api_key: Some(configuration::ApiKey{
            prefix: None,
            key: token.to_string(),
        }),
        ..configuration::Configuration::new()
    };

    let result = sync_api::current_block_id(&cg, platform, network).await;
    match result {
        Ok(id) => println!("current block: {}", id),
        Err(e) => panic!("{}", e),
    }

    let balance_map =
        accounts_api::get_list_of_balances_by_address(&cg, platform, network, addr, None).await;

    match balance_map {
        Ok(b) => {
            let f = b.first().unwrap();
            println!("Balance: {:?}", f);

            Some(println!("ok"));
        }
        Err(e) => panic!("{}", e),
    }

    let utxos_result = accounts_api::get_utxoby_account(&cg, platform, network, addr, Some(false), None, None, None, None, Some(10)).await;
    match utxos_result {
        Ok(utxos) => {
            println!("Total UTXO: {:?} - Continuation: {:?}", utxos.total.unwrap(), utxos.continuation);
            for u in utxos.data.unwrap_or_else(Vec::new).iter() {
                println!("{:?}", u);
            }

            Some(println!("ok"));
        }
        Err(e) => panic!("{}", e),
    }
}
