use crate::blockchain::contract_abi::{BlockchainPolicy, BuyPolicyTransaction, WeatherInsurance};
use crate::db::models::CreateInsurancePolicyRequest;
use ethers::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;

// Error types for blockchain verification
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("Transaction not confirmed")]
    TransactionNotConfirmed,
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    #[error("Parameter mismatch: {0}")]
    ParameterMismatch(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Contract error: {0}")]
    ContractError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

// Result of blockchain verification
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub verified: bool,
    pub block_number: Option<u64>,
    pub error_message: Option<String>,
    pub blockchain_policy: Option<BlockchainPolicy>,
}

// Configuration for blockchain service
#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    pub rpc_url: String,
    pub contract_address: String,
    pub verification_enabled: bool,
    pub timeout_seconds: u64,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8545".to_string(),
            contract_address: "".to_string(),
            verification_enabled: true,
            timeout_seconds: 30,
        }
    }
}

pub struct BlockchainService {
    provider: Arc<Provider<Http>>,
    contract_address: Address,
    config: BlockchainConfig,
}

impl BlockchainService {
    pub fn new(config: BlockchainConfig) -> Result<Self, BlockchainError> {
        let provider = Provider::<Http>::try_from(&config.rpc_url).map_err(|e| {
            BlockchainError::NetworkError(format!("Failed to connect to RPC: {}", e))
        })?;

        let contract_address = Address::from_str(&config.contract_address)
            .map_err(|e| BlockchainError::ParseError(format!("Invalid contract address: {}", e)))?;

        Ok(Self {
            provider: Arc::new(provider),
            contract_address,
            config,
        })
    }

    // Main verification function - simplified for hackathon
    pub async fn verify_policy_transaction(
        &self,
        tx_hash: &str,
        user_wallet_address: &str,
        policy_request: &CreateInsurancePolicyRequest,
    ) -> Result<VerificationResult, BlockchainError> {
        if !self.config.verification_enabled {
            return Ok(VerificationResult {
                verified: true,
                block_number: None,
                error_message: Some("Verification disabled".to_string()),
                blockchain_policy: None,
            });
        }

        // Step 1: Verify transaction exists and is confirmed
        let (tx_receipt, block_number) = self.verify_transaction_confirmed(tx_hash).await?;

        // Step 2: Get transaction details
        let transaction = self.get_transaction_details(tx_hash).await?;

        // Step 3: Validate user address matches transaction sender
        let user_addr = Address::from_str(user_wallet_address)
            .map_err(|e| BlockchainError::ParseError(format!("Invalid user address: {}", e)))?;

        if transaction.from != user_addr {
            return Ok(VerificationResult {
                verified: false,
                block_number: Some(block_number),
                error_message: Some("Transaction sender does not match user wallet".to_string()),
                blockchain_policy: None,
            });
        }

        // Step 4: Verify transaction was sent to our contract
        if transaction.to != Some(self.contract_address) {
            return Ok(VerificationResult {
                verified: false,
                block_number: Some(block_number),
                error_message: Some(
                    "Transaction not sent to WeatherInsurance contract".to_string(),
                ),
                blockchain_policy: None,
            });
        }

        // Step 5: Basic validation - check if this looks like a buyPolicy transaction
        if transaction.value == U256::zero() {
            return Ok(VerificationResult {
                verified: false,
                block_number: Some(block_number),
                error_message: Some("No ETH sent with transaction".to_string()),
                blockchain_policy: None,
            });
        }

        // For hackathon: simplified verification passes if basic checks are met
        // In production, you would decode the full transaction parameters
        Ok(VerificationResult {
            verified: true,
            block_number: Some(block_number),
            error_message: None,
            blockchain_policy: None, // Could populate this by querying the contract
        })
    }

    // Verify transaction exists and is confirmed
    async fn verify_transaction_confirmed(
        &self,
        tx_hash: &str,
    ) -> Result<(TransactionReceipt, u64), BlockchainError> {
        let tx_hash = H256::from_str(tx_hash)
            .map_err(|e| BlockchainError::ParseError(format!("Invalid transaction hash: {}", e)))?;

        let receipt = self
            .provider
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(|e| {
                BlockchainError::NetworkError(format!("Failed to get transaction receipt: {}", e))
            })?
            .ok_or_else(|| {
                BlockchainError::TransactionNotFound(format!("Transaction {} not found", tx_hash))
            })?;

        // Check if transaction was successful
        if receipt.status != Some(U64::from(1)) {
            return Err(BlockchainError::InvalidTransaction(
                "Transaction failed".to_string(),
            ));
        }

        let block_number = receipt
            .block_number
            .ok_or_else(|| BlockchainError::TransactionNotConfirmed)?
            .as_u64();

        Ok((receipt, block_number))
    }

    // Get transaction details
    async fn get_transaction_details(&self, tx_hash: &str) -> Result<Transaction, BlockchainError> {
        let tx_hash = H256::from_str(tx_hash)
            .map_err(|e| BlockchainError::ParseError(format!("Invalid transaction hash: {}", e)))?;

        let transaction = self
            .provider
            .get_transaction(tx_hash)
            .await
            .map_err(|e| {
                BlockchainError::NetworkError(format!("Failed to get transaction: {}", e))
            })?
            .ok_or_else(|| {
                BlockchainError::TransactionNotFound(format!("Transaction {} not found", tx_hash))
            })?;

        Ok(transaction)
    }

    // Check if a transaction hash has already been used
    pub async fn is_transaction_used(&self, _tx_hash: &str) -> Result<bool, BlockchainError> {
        // This would query the database to check if the transaction hash has been used
        // For now, return false (not implemented)
        Ok(false)
    }

    // Simple health check for the blockchain service
    pub async fn health_check(&self) -> Result<bool, BlockchainError> {
        match self.provider.get_block_number().await {
            Ok(_) => Ok(true),
            Err(e) => Err(BlockchainError::NetworkError(format!(
                "Health check failed: {}",
                e
            ))),
        }
    }
}
