//! general eth api tests with websocket provider

use alloy_primitives::U256;
use alloy_provider::Provider;
use anvil::{spawn, NodeConfig};

#[tokio::test(flavor = "multi_thread")]
async fn can_get_block_number_ws() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let block_num = api.block_number().unwrap();
    println!("block_num: {}", block_num);

    assert_eq!(block_num, U256::ZERO);

    let provider = handle.ws_provider();
    println!("ws endpoint: {}", handle.ws_endpoint());
    // Output:
    // ws endpoint: ws://127.0.0.1:35913
    // It is NOT 8545

    println!("http_endpoint: {}", handle.http_endpoint());
    // Output: 
    // http_endpoint: http://127.0.0.1:44703
    // Identical to the ws endpoint



    let num = provider.get_block_number().await.unwrap();
    assert_eq!(num, block_num.to::<u64>());
    println!("num: {}", num);

}

#[tokio::test(flavor = "multi_thread")]
async fn can_dev_get_balance_ws() {
    let (_api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.ws_provider();

    let genesis_balance = handle.genesis_balance();
    for acc in handle.genesis_accounts() {
        let balance = provider.get_balance(acc).await.unwrap();
        assert_eq!(balance, genesis_balance);
    }
}
