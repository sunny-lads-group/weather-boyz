use dotenv::dotenv;
use ethers::prelude::*;
use std::env;
use std::sync::Arc;

abigen!(
    WeatherInsurance,
    r#"[
        function trigger(uint256, int256) external
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
    let policy_id = U256::from(0);
    let observed_temp = I256::from(-5);

    // Split into separate steps as suggested by compiler
    let call = contract.trigger(policy_id, observed_temp);
    let pending_tx = call.send().await?;

    println!("TX sent: {:?}", pending_tx.tx_hash());

    // Wait for confirmation
    let receipt = pending_tx.await?;
    println!("Receipt: {:?}", receipt);

    Ok(())
}
