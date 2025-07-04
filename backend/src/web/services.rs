use axum::{Extension, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Postgres;

use crate::db::models::{User, PolicyTemplate, CreateInsurancePolicy, CreateInsurancePolicyRequest, InsurancePolicy};
use crate::db::policy_queries;
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
            tracing::info!("Retrieved {} policy templates for user {}", templates.len(), current_user.email);
            Json(templates).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch policy templates: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch policy templates").into_response()
        }
    }
}

pub async fn create_policy(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
    Json(request_data): Json<CreateInsurancePolicyRequest>,
) -> impl IntoResponse {
    tracing::info!("Creating policy '{}' for user {}", request_data.policy_name, current_user.email);
    
    // Convert request struct to database struct with user_id from JWT
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
    
    match policy_queries::create_insurance_policy(&pool, &policy_data).await {
        Ok(policy) => {
            tracing::info!("Successfully created policy with id: {} for user {}", policy.id, current_user.email);
            (StatusCode::CREATED, Json(policy)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create policy for user {}: {}", current_user.email, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create policy").into_response()
        }
    }
}

pub async fn get_user_policies(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {
    tracing::info!("Fetching policies for user {}", current_user.email);
    
    match policy_queries::get_policies_by_user_id(&pool, current_user.id).await {
        Ok(policies) => {
            tracing::info!("Retrieved {} policies for user {}", policies.len(), current_user.email);
            Json(policies).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch policies for user {}: {}", current_user.email, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch policies").into_response()
        }
    }
}
