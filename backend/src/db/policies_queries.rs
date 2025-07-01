use crate::db::models::{AvailablePolicy};
use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use tracing;
use sqlx::{Error as SqlxError, Pool, Postgres};
use chrono::{Utc, DateTime};

pub async fn get_available_policies(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<AvailablePolicy>>, (StatusCode, Json<serde_json::Value>)> {
    let policies = sqlx::query_as!(
        AvailablePolicy,
        r#"
        SELECT id, name, description, payout, duration, event_type, threshold
        FROM available_policies
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to fetch available policies: {}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to fetch available policies"
            })),
        )
    })?;
    Ok(Json(policies))
}

pub async fn seed_available_policy(pool: &Pool<Postgres>,) -> Result<(), SqlxError> {
    // Check if any policies already exist in the table
    let count = sqlx::query!("SELECT COUNT(*) AS count FROM available_policies")
        .fetch_one(pool)
        .await?
        .count;
    if count.unwrap_or(0) > 0 {
        tracing::info!("Policies already exist, skipping seed");
        return Ok(());
    }

    // Seed data for HOT and COLD policies
    let seed_data = vec![
        (
            "HOT".to_string(),
            "Hot policy description".to_string(),
            150.0,
            7,
            "temperature".to_string(),
            45.0, // Threshold for HOT
        ),
        (
            "COLD".to_string(),
            "Cold policy description".to_string(),
            120.0,
            7,
            "temperature".to_string(),
            -1.0, // Threshold for COLD
        ),
    ];

    for (name, description, payout, duration, event_type, threshold) in seed_data {
        sqlx::query!(
            r#"
            INSERT INTO available_policies (
                name, description, payout, duration, event_type, threshold
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            name,
            description,
            payout,
            duration,
            event_type,
            threshold,
        )
        .execute(pool)
        .await?;
    }

    tracing::info!("Database seeded with HOT and COLD policies");
    Ok(())
}