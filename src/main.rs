extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate dotenv;
use aptos_sdk::rest_client::{
    self,
    //FaucetClient,
    Transaction,
};
use dotenv::dotenv;
use oracle_sdk::{oracle, OracleClient, OracleConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    let oracle_client = OracleClient::new(rest_client::Client::new(oracle::NODE_URL.clone()));

    let default_account =
        &mut OracleConfig::load_default_account(&oracle_client, "./.aptos/config.yaml")
            .await
            .unwrap();

    //let faucet_client = FaucetClient::new(oracle::FAUCET_URL.clone(), oracle::NODE_URL.clone());
    //faucet_client.fund(oracle_account.address(), 5_000).await?;

    let result = oracle_client
        .api_client
        .get_account_balance(default_account.address())
        .await?
        .inner()
        .get();

    println!("The balance is {}", result);

    /****************************init aggregator ********************************/
    // let pending_transaction = oracle_client
    //     .initialize_aggregator(
    //         default_account.address(),
    //         default_account,
    //         1,
    //         "Coinmarket Agggregator",
    //         None,
    //     )
    //     .await
    //     .unwrap()
    //     .into_inner();

    /****************************init token ********************************/

    // let pending_transaction = oracle_client
    //     .initialize_token(
    //         default_account.address(),
    //         default_account,
    //         "Ethereum",
    //         "ETH",
    //         None,
    //     )
    //     .await
    //     .unwrap()
    //     .into_inner();

    /****************************add feed ********************************/
    // let pending_transaction = oracle_client
    //     .add_feed(
    //         default_account.address(),
    //         default_account,
    //         "ETH",
    //         900000,
    //         3,
    //         "20220927",
    //         None,
    //     )
    //     .await
    //     .unwrap()
    //     .into_inner();

    /****************************get feed (NOT WORKING YET IN APTOS) ********************************/
    // let pending_transaction = oracle_client
    //     .get_feed(default_account.address(), default_account, "ETH", None)
    //     .await
    //     .unwrap()
    //     .into_inner();

    /*********************** Start of Transaction *********************/
    // let result: Transaction = oracle_client
    //     .api_client
    //     .wait_for_transaction(&pending_transaction)
    //     .await?
    //     .into_inner();

    // println!("The transaction is {}", result.success());

    /*********************** End of Transaction *********************/

    // let _ = oracle_client
    //     .get_aggregator_data(default_account)
    //     .await
    //     .unwrap();

    // let result = oracle_client
    //     .get_latest_token_data(default_account, "ETH")
    //     .await
    //     .unwrap();

    // info!("{:?}", result);

    Ok(())
}
