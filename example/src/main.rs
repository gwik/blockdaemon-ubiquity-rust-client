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
        bearer_access_token: Some(token),
        ..configuration::Configuration::new()
    };

    let result = sync_api::current_block_id(&cg, platform, network).await;
    match result {
        Ok(id) => println!("current block: {}", id),
        Err(e) => panic!("{}", e),
    }

    let balance_map =
        accounts_api::get_list_of_balances_by_address(&cg, platform, network, addr).await;

    match balance_map {
        Ok(b) => {
            let f = b.first().unwrap();
            println!("Balance:{:?}", f);

            Some(println!("ok"));
        }
        Err(e) => panic!("{}", e),
    }
}
