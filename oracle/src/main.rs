// This is just starter code.


use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

abigen!(
    WeatherInsurance,
    r#"[
        function triggerPayout(uint256, int256, uint256) external
    ]"#,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Environment
    let private_key = env::var("PRIVATE_KEY")?;
    let rpc_url = env::var("RPC_URL")?;
    let contract_addr: Address = env::var("CONTRACT_ADDRESS")?.parse()?;
    let chain_id: u64 = env::var("CHAIN_ID")?.parse()?;

    // Wallet + Provider
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Contract instance
    let contract = WeatherInsurance::new(contract_addr, client.clone());

    // Simulate oracle reading
    let policy_id: u64 = 0;
    let observed_temp: i64 = -5;
    let obs_time: u64 = 1712345678;

    let tx = contract
        .trigger_payout(policy_id, observed_temp, obs_time)
        .send()
        .await?;

    println!("TX sent: {:?}", tx.tx_hash());

    let receipt = tx.await?;
    println!("Receipt: {:?}", receipt);

    Ok(())
}