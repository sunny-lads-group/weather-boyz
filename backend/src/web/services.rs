// This file contains all exposed services for the backend
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Postgres;

use crate::blockchain::{BlockchainConfig, BlockchainService};
use crate::db::models::{
    CreateInsurancePolicy, CreateInsurancePolicyRequest, InsurancePolicy, PolicyTemplate, User,
};
use crate::db::{policy_queries, user_queries};
use crate::web::{
    auth::{self},
    services,
};
#[derive(Serialize, Deserialize)]
struct UserResponse {
    email: String,
    name: String,
}

pub async fn hello(Extension(current_user): Extension<User>) -> impl IntoResponse {
    Json(UserResponse {
        email: current_user.email,
        name: current_user.name,
    })
}

pub async fn get_policy_templates(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {
    match policy_queries::get_all_policy_templates(&pool).await {
        Ok(templates) => {
            tracing::info!(
                "Retrieved {} policy templates for user {}",
                templates.len(),
                current_user.email
            );
            Json(templates).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch policy templates: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch policy templates",
            )
                .into_response()
        }
    }
}

pub async fn create_policy(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
    Json(request_data): Json<CreateInsurancePolicyRequest>,
) -> impl IntoResponse {
    tracing::info!(
        "Creating policy '{}' for user {}",
        request_data.policy_name,
        current_user.email
    );

    // Validate that user has a wallet address
    let user_wallet_address = match &current_user.wallet_address {
        Some(addr) => addr,
        None => {
            tracing::warn!(
                "User {} attempted to create policy without wallet address",
                current_user.email
            );
            return (
                StatusCode::BAD_REQUEST,
                "User wallet address not found. Please connect your wallet first.",
            )
                .into_response();
        }
    };

    // Validate that transaction hash is provided
    let tx_hash = match &request_data.purchase_transaction_hash {
        Some(hash) => hash,
        None => {
            tracing::warn!(
                "Policy creation attempted without transaction hash for user {}",
                current_user.email
            );
            return (
                StatusCode::BAD_REQUEST,
                "Purchase transaction hash is required",
            )
                .into_response();
        }
    };

    // Perform blockchain verification
    let blockchain_config = get_blockchain_config();
    let blockchain_service = match BlockchainService::new(blockchain_config) {
        Ok(service) => service,
        Err(e) => {
            tracing::error!("Failed to initialize blockchain service: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Blockchain verification service unavailable",
            )
                .into_response();
        }
    };

    let verification_result = match blockchain_service
        .verify_policy_transaction(tx_hash, user_wallet_address, &request_data)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!(
                "Blockchain verification failed for user {}: {}",
                current_user.email,
                e
            );
            return (
                StatusCode::BAD_REQUEST,
                format!("Blockchain verification failed: {}", e),
            )
                .into_response();
        }
    };

    if !verification_result.verified {
        let error_msg = verification_result
            .error_message
            .unwrap_or("Verification failed".to_string());
        tracing::warn!(
            "Blockchain verification failed for user {}: {}",
            current_user.email,
            error_msg
        );
        return (
            StatusCode::BAD_REQUEST,
            format!("Blockchain verification failed: {}", error_msg),
        )
            .into_response();
    }

    tracing::info!(
        "Blockchain verification successful for user {}",
        current_user.email
    );

    // Convert request struct to database struct with user_id from JWT and verification data
    let policy_data = CreateInsurancePolicy {
        user_id: current_user.id,
        policy_template_id: request_data.policy_template_id,
        policy_name: request_data.policy_name,
        policy_type: request_data.policy_type,
        location_latitude: request_data.location_latitude,
        location_longitude: request_data.location_longitude,
        location_h3_index: request_data.location_h3_index,
        location_name: request_data.location_name,
        coverage_amount: request_data.coverage_amount,
        premium_amount: request_data.premium_amount,
        currency: request_data.currency,
        start_date: request_data.start_date,
        end_date: request_data.end_date,
        weather_station_id: request_data.weather_station_id,
        smart_contract_address: request_data.smart_contract_address,
        purchase_transaction_hash: request_data.purchase_transaction_hash,
    };

    match policy_queries::create_insurance_policy_with_verification(
        &pool,
        &policy_data,
        &verification_result,
    )
    .await
    {
        Ok(policy) => {
            tracing::info!(
                "Successfully created verified policy with id: {} for user {}",
                policy.id,
                current_user.email
            );
            (StatusCode::CREATED, Json(policy)).into_response()
        }
        Err(e) => {
            tracing::error!(
                "Failed to create policy for user {}: {}",
                current_user.email,
                e
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create policy").into_response()
        }
    }
}

// Helper function to get blockchain configuration from environment
fn get_blockchain_config() -> BlockchainConfig {
    BlockchainConfig {
        rpc_url: std::env::var("ETHEREUM_RPC_URL")
            .unwrap_or_else(|_| "http://localhost:8545".to_string()),
        contract_address: std::env::var("WEATHER_INSURANCE_CONTRACT_ADDRESS").unwrap_or_default(),
        verification_enabled: std::env::var("BLOCKCHAIN_VERIFICATION_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            == "true",
        timeout_seconds: std::env::var("VERIFICATION_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30),
    }
}

pub async fn get_user_policies(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {
    tracing::info!("Fetching policies for user {}", current_user.email);

    match policy_queries::get_policies_by_user_id(&pool, current_user.id).await {
        Ok(policies) => {
            tracing::info!(
                "Retrieved {} policies for user {}",
                policies.len(),
                current_user.email
            );
            Json(policies).into_response()
        }
        Err(e) => {
            tracing::error!(
                "Failed to fetch policies for user {}: {}",
                current_user.email,
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch policies",
            )
                .into_response()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateWalletAddressRequest {
    wallet_address: String,
}

pub async fn update_wallet_address(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
    Json(request): Json<UpdateWalletAddressRequest>,
) -> impl IntoResponse {
    tracing::info!("Updating wallet address for user {}", current_user.email);

    // Validate wallet address format (basic Ethereum address validation)
    if !is_valid_ethereum_address(&request.wallet_address) {
        tracing::warn!(
            "Invalid wallet address format provided by user {}",
            current_user.email
        );
        return (StatusCode::BAD_REQUEST, "Invalid wallet address format").into_response();
    }

    match user_queries::update_user_wallet_address(&pool, current_user.id, &request.wallet_address)
        .await
    {
        Ok(updated_user) => {
            tracing::info!(
                "Successfully updated wallet address for user {}",
                current_user.email
            );
            Json(UserResponse {
                email: updated_user.email,
                name: updated_user.name,
            })
            .into_response()
        }
        Err(e) => {
            tracing::error!(
                "Failed to update wallet address for user {}: {}",
                current_user.email,
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update wallet address",
            )
                .into_response()
        }
    }
}

// Helper function to validate Ethereum address format
fn is_valid_ethereum_address(address: &str) -> bool {
    // Basic validation: starts with 0x and is 42 characters long
    address.starts_with("0x")
        && address.len() == 42
        && address.chars().skip(2).all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ethereum_addresses() {
        // Test basic valid address format
        assert!(is_valid_ethereum_address(
            "0x1234567890123456789012345678901234567890"
        ));

        // Test mixed case works
        assert!(is_valid_ethereum_address(
            "0x1234567890abcdef123456789012345678901234"
        ));
        assert!(is_valid_ethereum_address(
            "0x1234567890ABCDEF123456789012345678901234"
        ));

        // Test edge cases
        assert!(is_valid_ethereum_address(
            "0x0000000000000000000000000000000000000000"
        ));
        assert!(is_valid_ethereum_address(
            "0xffffffffffffffffffffffffffffffffffffffff"
        ));
        assert!(is_valid_ethereum_address(
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ));
    }

    #[test]
    fn test_invalid_ethereum_addresses() {
        // Missing 0x prefix
        assert!(!is_valid_ethereum_address(
            "1234567890123456789012345678901234567890"
        ));

        // Too short
        assert!(!is_valid_ethereum_address("0x123"));
        assert!(!is_valid_ethereum_address(
            "0x123456789012345678901234567890123456789"
        ));

        // Too long
        assert!(!is_valid_ethereum_address(
            "0x12345678901234567890123456789012345678901"
        ));

        // Invalid characters
        assert!(!is_valid_ethereum_address(
            "0x123456789012345678901234567890123456789G"
        ));
        assert!(!is_valid_ethereum_address(
            "0x123456789012345678901234567890123456789g"
        ));
        assert!(!is_valid_ethereum_address(
            "0x12345678901234567890123456789012345678-0"
        ));

        // Empty string
        assert!(!is_valid_ethereum_address(""));

        // Just 0x
        assert!(!is_valid_ethereum_address("0x"));

        // Wrong prefix
        assert!(!is_valid_ethereum_address(
            "1x1234567890123456789012345678901234567890"
        ));
        assert!(!is_valid_ethereum_address(
            "0X1234567890123456789012345678901234567890"
        ));

        // Contains spaces
        assert!(!is_valid_ethereum_address(
            "0x1234567890123456789012345678901234567890 "
        ));
        assert!(!is_valid_ethereum_address(
            " 0x1234567890123456789012345678901234567890"
        ));
    }
}
