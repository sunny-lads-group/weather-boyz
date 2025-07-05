use crate::db::models::*;
use sqlx::{Pool, Postgres};
use tracing::{debug, info};

// ============================================================================
// POLICY TEMPLATE QUERIES
// ============================================================================

pub async fn get_all_policy_templates(
    pool: &Pool<Postgres>,
) -> Result<Vec<PolicyTemplate>, sqlx::Error> {
    debug!("Fetching all active policy templates");

    let templates = sqlx::query_as!(
        PolicyTemplate,
        "SELECT id, template_name, description, policy_type, default_conditions, 
         min_coverage_amount, max_coverage_amount, base_premium_rate, is_active, 
         created_at, updated_at 
         FROM policy_templates 
         WHERE is_active = true
         ORDER BY template_name"
    )
    .fetch_all(pool)
    .await?;

    info!("Retrieved {} policy templates", templates.len());
    Ok(templates)
}

pub async fn get_policy_template_by_id(
    pool: &Pool<Postgres>,
    template_id: i32,
) -> Result<Option<PolicyTemplate>, sqlx::Error> {
    debug!("Fetching policy template with id: {}", template_id);

    let template = sqlx::query_as!(
        PolicyTemplate,
        "SELECT id, template_name, description, policy_type, default_conditions, 
         min_coverage_amount, max_coverage_amount, base_premium_rate, is_active, 
         created_at, updated_at 
         FROM policy_templates 
         WHERE id = $1",
        template_id
    )
    .fetch_optional(pool)
    .await?;

    if template.is_some() {
        info!("Found policy template with id: {}", template_id);
    } else {
        debug!("No policy template found with id: {}", template_id);
    }

    Ok(template)
}

pub async fn create_policy_template(
    pool: &Pool<Postgres>,
    template_data: &CreatePolicyTemplate,
) -> Result<PolicyTemplate, sqlx::Error> {
    info!(
        "Creating new policy template: {}",
        template_data.template_name
    );

    let template = sqlx::query_as!(
        PolicyTemplate,
        "INSERT INTO policy_templates 
         (template_name, description, policy_type, default_conditions, min_coverage_amount, max_coverage_amount, base_premium_rate)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, template_name, description, policy_type, default_conditions, 
         min_coverage_amount, max_coverage_amount, base_premium_rate, is_active, created_at, updated_at",
        template_data.template_name,
        template_data.description,
        template_data.policy_type,
        template_data.default_conditions,
        template_data.min_coverage_amount,
        template_data.max_coverage_amount,
        template_data.base_premium_rate
    )
    .fetch_one(pool)
    .await?;

    info!("Created policy template with id: {}", template.id);
    Ok(template)
}

// ============================================================================
// INSURANCE POLICY QUERIES
// ============================================================================

pub async fn get_policies_by_user_id(
    pool: &Pool<Postgres>,
    user_id: i32,
) -> Result<Vec<InsurancePolicy>, sqlx::Error> {
    debug!("Fetching policies for user id: {}", user_id);

    let policies = sqlx::query_as!(
        InsurancePolicy,
        "SELECT id, user_id, policy_template_id, policy_name, policy_type,
         location_latitude, location_longitude, location_h3_index, location_name,
         coverage_amount, premium_amount, currency, start_date, end_date, status,
         weather_station_id, smart_contract_address, purchase_transaction_hash,
         blockchain_verified, verification_timestamp, blockchain_block_number, verification_error_message,
         created_at, updated_at
         FROM insurance_policies 
         WHERE user_id = $1
         ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(pool)
    .await?;

    info!("Retrieved {} policies for user {}", policies.len(), user_id);
    Ok(policies)
}

pub async fn get_policy_by_id(
    pool: &Pool<Postgres>,
    policy_id: i32,
) -> Result<Option<InsurancePolicy>, sqlx::Error> {
    debug!("Fetching policy with id: {}", policy_id);

    let policy = sqlx::query_as!(
        InsurancePolicy,
        "SELECT id, user_id, policy_template_id, policy_name, policy_type,
         location_latitude, location_longitude, location_h3_index, location_name,
         coverage_amount, premium_amount, currency, start_date, end_date, status,
         weather_station_id, smart_contract_address, purchase_transaction_hash,
         blockchain_verified, verification_timestamp, blockchain_block_number, verification_error_message,
         created_at, updated_at
         FROM insurance_policies 
         WHERE id = $1",
        policy_id
    )
    .fetch_optional(pool)
    .await?;

    if policy.is_some() {
        info!("Found policy with id: {}", policy_id);
    } else {
        debug!("No policy found with id: {}", policy_id);
    }

    Ok(policy)
}

pub async fn create_insurance_policy(
    pool: &Pool<Postgres>,
    policy_data: &CreateInsurancePolicy,
) -> Result<InsurancePolicy, sqlx::Error> {
    info!(
        "Creating new insurance policy: {} for user {}",
        policy_data.policy_name, policy_data.user_id
    );

    let currency = policy_data.currency.as_deref().unwrap_or("ETH");

    let policy = sqlx::query_as!(
        InsurancePolicy,
        "INSERT INTO insurance_policies 
         (user_id, policy_template_id, policy_name, policy_type, location_latitude, location_longitude,
          location_h3_index, location_name, coverage_amount, premium_amount, currency, start_date, end_date,
          weather_station_id, smart_contract_address, purchase_transaction_hash)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
         RETURNING id, user_id, policy_template_id, policy_name, policy_type,
         location_latitude, location_longitude, location_h3_index, location_name,
         coverage_amount, premium_amount, currency, start_date, end_date, status,
         weather_station_id, smart_contract_address, purchase_transaction_hash,
         blockchain_verified, verification_timestamp, blockchain_block_number, verification_error_message,
         created_at, updated_at",
        policy_data.user_id,
        policy_data.policy_template_id,
        policy_data.policy_name,
        policy_data.policy_type,
        policy_data.location_latitude,
        policy_data.location_longitude,
        policy_data.location_h3_index,
        policy_data.location_name,
        policy_data.coverage_amount,
        policy_data.premium_amount,
        currency,
        policy_data.start_date,
        policy_data.end_date,
        policy_data.weather_station_id,
        policy_data.smart_contract_address,
        policy_data.purchase_transaction_hash
    )
    .fetch_one(pool)
    .await?;

    info!("Created insurance policy with id: {}", policy.id);
    Ok(policy)
}

pub async fn create_insurance_policy_with_verification(
    pool: &Pool<Postgres>,
    policy_data: &CreateInsurancePolicy,
    verification_result: &crate::blockchain::VerificationResult,
) -> Result<InsurancePolicy, sqlx::Error> {
    info!(
        "Creating new verified insurance policy: {} for user {}",
        policy_data.policy_name, policy_data.user_id
    );

    let currency = policy_data.currency.as_deref().unwrap_or("ETH");

    let policy = sqlx::query_as!(
        InsurancePolicy,
        "INSERT INTO insurance_policies 
         (user_id, policy_template_id, policy_name, policy_type, location_latitude, location_longitude,
          location_h3_index, location_name, coverage_amount, premium_amount, currency, start_date, end_date,
          weather_station_id, smart_contract_address, purchase_transaction_hash,
          blockchain_verified, verification_timestamp, blockchain_block_number, verification_error_message)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, CURRENT_TIMESTAMP, $18, $19)
         RETURNING id, user_id, policy_template_id, policy_name, policy_type,
         location_latitude, location_longitude, location_h3_index, location_name,
         coverage_amount, premium_amount, currency, start_date, end_date, status,
         weather_station_id, smart_contract_address, purchase_transaction_hash,
         blockchain_verified, verification_timestamp, blockchain_block_number, verification_error_message,
         created_at, updated_at",
        policy_data.user_id,
        policy_data.policy_template_id,
        policy_data.policy_name,
        policy_data.policy_type,
        policy_data.location_latitude,
        policy_data.location_longitude,
        policy_data.location_h3_index,
        policy_data.location_name,
        policy_data.coverage_amount,
        policy_data.premium_amount,
        currency,
        policy_data.start_date,
        policy_data.end_date,
        policy_data.weather_station_id,
        policy_data.smart_contract_address,
        policy_data.purchase_transaction_hash,
        verification_result.verified,
        verification_result.block_number.map(|n| n as i64),
        verification_result.error_message.as_deref()
    )
    .fetch_one(pool)
    .await?;

    info!("Created verified insurance policy with id: {}", policy.id);
    Ok(policy)
}

pub async fn update_policy_status(
    pool: &Pool<Postgres>,
    policy_id: i32,
    new_status: &str,
) -> Result<bool, sqlx::Error> {
    info!("Updating policy {} status to: {}", policy_id, new_status);

    let result = sqlx::query!(
        "UPDATE insurance_policies SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
        new_status,
        policy_id
    )
    .execute(pool)
    .await?;

    let updated = result.rows_affected() > 0;
    if updated {
        info!("Successfully updated policy {} status", policy_id);
    } else {
        debug!("No policy found with id: {}", policy_id);
    }

    Ok(updated)
}

// ============================================================================
// POLICY CONDITION QUERIES
// ============================================================================

pub async fn get_conditions_by_policy_id(
    pool: &Pool<Postgres>,
    policy_id: i32,
) -> Result<Vec<PolicyCondition>, sqlx::Error> {
    debug!("Fetching conditions for policy id: {}", policy_id);

    let conditions = sqlx::query_as!(
        PolicyCondition,
        "SELECT id, policy_id, condition_type, operator, threshold_value,
         measurement_unit, measurement_period, consecutive_days, created_at
         FROM policy_conditions 
         WHERE policy_id = $1
         ORDER BY id",
        policy_id
    )
    .fetch_all(pool)
    .await?;

    debug!(
        "Retrieved {} conditions for policy {}",
        conditions.len(),
        policy_id
    );
    Ok(conditions)
}

pub async fn create_policy_condition(
    pool: &Pool<Postgres>,
    condition_data: &CreatePolicyCondition,
) -> Result<PolicyCondition, sqlx::Error> {
    debug!(
        "Creating new policy condition for policy id: {}",
        condition_data.policy_id
    );

    let consecutive_days = condition_data.consecutive_days.unwrap_or(1);

    let condition = sqlx::query_as!(
        PolicyCondition,
        "INSERT INTO policy_conditions 
         (policy_id, condition_type, operator, threshold_value, measurement_unit, measurement_period, consecutive_days)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, policy_id, condition_type, operator, threshold_value,
         measurement_unit, measurement_period, consecutive_days, created_at",
        condition_data.policy_id,
        condition_data.condition_type,
        condition_data.operator,
        condition_data.threshold_value,
        condition_data.measurement_unit,
        condition_data.measurement_period,
        consecutive_days
    )
    .fetch_one(pool)
    .await?;

    debug!("Created policy condition with id: {}", condition.id);
    Ok(condition)
}

// ============================================================================
// WEATHER DATA QUERIES
// ============================================================================

pub async fn insert_weather_data(
    pool: &Pool<Postgres>,
    weather_data: &CreateWeatherData,
) -> Result<WeatherData, sqlx::Error> {
    debug!(
        "Inserting weather data for station: {} at {}",
        weather_data.station_id, weather_data.recorded_at
    );

    let data_source = weather_data.data_source.as_deref().unwrap_or("weatherxm");

    let data = sqlx::query_as!(
        WeatherData,
        "INSERT INTO weather_data 
         (station_id, recorded_at, temperature, humidity, precipitation, wind_speed, 
          wind_direction, atmospheric_pressure, data_source, raw_data, quality_score)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
         ON CONFLICT (station_id, recorded_at) 
         DO UPDATE SET 
           temperature = EXCLUDED.temperature,
           humidity = EXCLUDED.humidity,
           precipitation = EXCLUDED.precipitation,
           wind_speed = EXCLUDED.wind_speed,
           wind_direction = EXCLUDED.wind_direction,
           atmospheric_pressure = EXCLUDED.atmospheric_pressure,
           raw_data = EXCLUDED.raw_data,
           quality_score = EXCLUDED.quality_score
         RETURNING id, station_id, recorded_at, temperature, humidity, precipitation,
         wind_speed, wind_direction, atmospheric_pressure, data_source, raw_data, quality_score, created_at",
        weather_data.station_id,
        weather_data.recorded_at,
        weather_data.temperature,
        weather_data.humidity,
        weather_data.precipitation,
        weather_data.wind_speed,
        weather_data.wind_direction,
        weather_data.atmospheric_pressure,
        data_source,
        weather_data.raw_data,
        weather_data.quality_score
    )
    .fetch_one(pool)
    .await?;

    debug!("Inserted/updated weather data with id: {}", data.id);
    Ok(data)
}

pub async fn get_weather_data_by_station_and_date_range(
    pool: &Pool<Postgres>,
    station_id: &str,
    start_date: &sqlx::types::time::PrimitiveDateTime,
    end_date: &sqlx::types::time::PrimitiveDateTime,
) -> Result<Vec<WeatherData>, sqlx::Error> {
    debug!(
        "Fetching weather data for station {} from {} to {}",
        station_id, start_date, end_date
    );

    let data = sqlx::query_as!(
        WeatherData,
        "SELECT id, station_id, recorded_at, temperature, humidity, precipitation,
         wind_speed, wind_direction, atmospheric_pressure, data_source, raw_data, quality_score, created_at
         FROM weather_data 
         WHERE station_id = $1 AND recorded_at >= $2 AND recorded_at <= $3
         ORDER BY recorded_at",
        station_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;

    debug!(
        "Retrieved {} weather data records for station {}",
        data.len(),
        station_id
    );
    Ok(data)
}

// ============================================================================
// POLICY CLAIM QUERIES
// ============================================================================

pub async fn get_claims_by_policy_id(
    pool: &Pool<Postgres>,
    policy_id: i32,
) -> Result<Vec<PolicyClaim>, sqlx::Error> {
    debug!("Fetching claims for policy id: {}", policy_id);

    let claims = sqlx::query_as!(
        PolicyClaim,
        "SELECT id, policy_id, claim_amount, claim_status, trigger_date,
         trigger_period_start, trigger_period_end, verification_data,
         evaluated_at, approved_at, rejected_at, rejection_reason,
         payout_transaction_hash, payout_block_number, created_at, updated_at
         FROM policy_claims 
         WHERE policy_id = $1
         ORDER BY created_at DESC",
        policy_id
    )
    .fetch_all(pool)
    .await?;

    debug!("Retrieved {} claims for policy {}", claims.len(), policy_id);
    Ok(claims)
}

pub async fn create_policy_claim(
    pool: &Pool<Postgres>,
    claim_data: &CreatePolicyClaim,
) -> Result<PolicyClaim, sqlx::Error> {
    info!(
        "Creating new policy claim for policy id: {}",
        claim_data.policy_id
    );

    let claim = sqlx::query_as!(
        PolicyClaim,
        "INSERT INTO policy_claims 
         (policy_id, claim_amount, trigger_date, trigger_period_start, trigger_period_end, verification_data)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, policy_id, claim_amount, claim_status, trigger_date,
         trigger_period_start, trigger_period_end, verification_data,
         evaluated_at, approved_at, rejected_at, rejection_reason,
         payout_transaction_hash, payout_block_number, created_at, updated_at",
        claim_data.policy_id,
        claim_data.claim_amount,
        claim_data.trigger_date,
        claim_data.trigger_period_start,
        claim_data.trigger_period_end,
        claim_data.verification_data
    )
    .fetch_one(pool)
    .await?;

    info!("Created policy claim with id: {}", claim.id);
    Ok(claim)
}

pub async fn update_claim_status(
    pool: &Pool<Postgres>,
    claim_id: i32,
    new_status: &str,
    rejection_reason: Option<&str>,
) -> Result<bool, sqlx::Error> {
    info!("Updating claim {} status to: {}", claim_id, new_status);

    let result = match new_status {
        "approved" => {
            sqlx::query!(
                "UPDATE policy_claims SET claim_status = $1, approved_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                new_status,
                claim_id
            )
            .execute(pool)
            .await?
        },
        "rejected" => {
            sqlx::query!(
                "UPDATE policy_claims SET claim_status = $1, rejected_at = CURRENT_TIMESTAMP, rejection_reason = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
                new_status,
                rejection_reason,
                claim_id
            )
            .execute(pool)
            .await?
        },
        _ => {
            sqlx::query!(
                "UPDATE policy_claims SET claim_status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                new_status,
                claim_id
            )
            .execute(pool)
            .await?
        }
    };

    let updated = result.rows_affected() > 0;
    if updated {
        info!(
            "Successfully updated claim {} status to {}",
            claim_id, new_status
        );
    } else {
        debug!("No claim found with id: {}", claim_id);
    }

    Ok(updated)
}

// ============================================================================
// COMBINED QUERIES FOR API RESPONSES
// ============================================================================

pub async fn get_policy_with_conditions(
    pool: &Pool<Postgres>,
    policy_id: i32,
) -> Result<Option<PolicyWithConditions>, sqlx::Error> {
    let policy = get_policy_by_id(pool, policy_id).await?;

    if let Some(policy) = policy {
        let conditions = get_conditions_by_policy_id(pool, policy_id).await?;
        Ok(Some(PolicyWithConditions { policy, conditions }))
    } else {
        Ok(None)
    }
}

pub async fn get_policy_with_claims(
    pool: &Pool<Postgres>,
    policy_id: i32,
) -> Result<Option<PolicyWithClaims>, sqlx::Error> {
    let policy = get_policy_by_id(pool, policy_id).await?;

    if let Some(policy) = policy {
        let claims = get_claims_by_policy_id(pool, policy_id).await?;
        Ok(Some(PolicyWithClaims { policy, claims }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_test_db;
    use rust_decimal::Decimal;
    use sqlx::types::time::PrimitiveDateTime;
    use std::str::FromStr;

    // Helper function to create a test user for policy tests
    async fn create_test_user_for_policies(pool: &Pool<Postgres>) -> i32 {
        let user = sqlx::query!(
            "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
            "Test User",
            "test@policies.com",
            "$2b$12$test_hash"
        )
        .fetch_one(pool)
        .await
        .expect("Failed to create test user");

        user.id
    }

    // ============================================================================
    // POLICY TEMPLATE TESTS
    // ============================================================================

    #[tokio::test]
    async fn test_get_all_policy_templates() {
        let test_db = create_test_db().await;

        let templates = get_all_policy_templates(&test_db.pool).await.unwrap();

        // Should return the 4 sample templates from migration
        assert_eq!(templates.len(), 4);

        let template_names: Vec<&str> =
            templates.iter().map(|t| t.template_name.as_str()).collect();
        assert!(template_names.contains(&"Drought Protection"));
        assert!(template_names.contains(&"Rain Event Insurance"));
        assert!(template_names.contains(&"Freeze Protection"));
        assert!(template_names.contains(&"Storm Insurance"));
    }

    #[tokio::test]
    async fn test_get_policy_template_by_id() {
        let test_db = create_test_db().await;

        // Get the first template
        let templates = get_all_policy_templates(&test_db.pool).await.unwrap();
        let first_template_id = templates[0].id;

        let template = get_policy_template_by_id(&test_db.pool, first_template_id)
            .await
            .unwrap();

        assert!(template.is_some());
        let template = template.unwrap();
        assert_eq!(template.id, first_template_id);
        assert!(!template.template_name.is_empty());
    }

    #[tokio::test]
    async fn test_get_policy_template_by_id_not_found() {
        let test_db = create_test_db().await;

        let template = get_policy_template_by_id(&test_db.pool, 99999)
            .await
            .unwrap();
        assert!(template.is_none());
    }

    #[tokio::test]
    async fn test_create_policy_template() {
        let test_db = create_test_db().await;

        let template_data = CreatePolicyTemplate {
            template_name: "Test Template".to_string(),
            description: Some("A test template".to_string()),
            policy_type: "test".to_string(),
            default_conditions: Some(serde_json::json!({"test": true})),
            min_coverage_amount: Decimal::from_str("100.00").unwrap(),
            max_coverage_amount: Decimal::from_str("5000.00").unwrap(),
            base_premium_rate: Decimal::from_str("0.05").unwrap(),
        };

        let created_template = create_policy_template(&test_db.pool, &template_data)
            .await
            .unwrap();

        assert_eq!(created_template.template_name, "Test Template");
        assert_eq!(created_template.policy_type, "test");
        assert_eq!(
            created_template.min_coverage_amount,
            Decimal::from_str("100.00").unwrap()
        );
        assert!(created_template.id > 0);
        assert_eq!(created_template.is_active, Some(true));
    }

    // ============================================================================
    // INSURANCE POLICY TESTS
    // ============================================================================

    #[tokio::test]
    async fn test_create_insurance_policy() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Test Policy".to_string(),
            policy_type: "drought".to_string(),
            location_latitude: Decimal::from_str("40.7128").unwrap(),
            location_longitude: Decimal::from_str("-74.0060").unwrap(),
            location_h3_index: Some("8a2a1072b59ffff".to_string()),
            location_name: Some("New York City".to_string()),
            coverage_amount: Decimal::from_str("1000.00").unwrap(),
            premium_amount: Decimal::from_str("50.00").unwrap(),
            currency: Some("ETH".to_string()),
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::December, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: Some("station123".to_string()),
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let created_policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        assert_eq!(created_policy.user_id, user_id);
        assert_eq!(created_policy.policy_name, "Test Policy");
        assert_eq!(created_policy.policy_type, "drought");
        assert_eq!(
            created_policy.coverage_amount,
            Decimal::from_str("1000.00").unwrap()
        );
        assert_eq!(created_policy.status, Some("active".to_string()));
        assert!(created_policy.id > 0);
    }

    #[tokio::test]
    async fn test_get_policies_by_user_id() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Initially no policies
        let policies = get_policies_by_user_id(&test_db.pool, user_id)
            .await
            .unwrap();
        assert_eq!(policies.len(), 0);

        // Create a policy
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "User Policy Test".to_string(),
            policy_type: "rain".to_string(),
            location_latitude: Decimal::from_str("40.7128").unwrap(),
            location_longitude: Decimal::from_str("-74.0060").unwrap(),
            location_h3_index: None,
            location_name: None,
            coverage_amount: Decimal::from_str("500.00").unwrap(),
            premium_amount: Decimal::from_str("25.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::December, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: None,
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        // Now should have one policy
        let policies = get_policies_by_user_id(&test_db.pool, user_id)
            .await
            .unwrap();
        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0].policy_name, "User Policy Test");
        assert_eq!(policies[0].user_id, user_id);
    }

    #[tokio::test]
    async fn test_get_policy_by_id() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "ID Test Policy".to_string(),
            policy_type: "temperature".to_string(),
            location_latitude: Decimal::from_str("34.0522").unwrap(),
            location_longitude: Decimal::from_str("-118.2437").unwrap(),
            location_h3_index: None,
            location_name: Some("Los Angeles".to_string()),
            coverage_amount: Decimal::from_str("2000.00").unwrap(),
            premium_amount: Decimal::from_str("100.00").unwrap(),
            currency: Some("USDC".to_string()),
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::June, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::August, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: Some("la_station".to_string()),
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let created_policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        let retrieved_policy = get_policy_by_id(&test_db.pool, created_policy.id)
            .await
            .unwrap();

        assert!(retrieved_policy.is_some());
        let retrieved_policy = retrieved_policy.unwrap();
        assert_eq!(retrieved_policy.id, created_policy.id);
        assert_eq!(retrieved_policy.policy_name, "ID Test Policy");
        assert_eq!(retrieved_policy.currency, Some("USDC".to_string()));
    }

    #[tokio::test]
    async fn test_update_policy_status() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Status Test Policy".to_string(),
            policy_type: "wind".to_string(),
            location_latitude: Decimal::from_str("41.8781").unwrap(),
            location_longitude: Decimal::from_str("-87.6298").unwrap(),
            location_h3_index: None,
            location_name: Some("Chicago".to_string()),
            coverage_amount: Decimal::from_str("1500.00").unwrap(),
            premium_amount: Decimal::from_str("75.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::March, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::November, 30).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: None,
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let created_policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();
        assert_eq!(created_policy.status, Some("active".to_string()));

        // Update status to expired
        let updated = update_policy_status(&test_db.pool, created_policy.id, "expired")
            .await
            .unwrap();
        assert!(updated);

        // Verify the status was updated
        let retrieved_policy = get_policy_by_id(&test_db.pool, created_policy.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_policy.status, Some("expired".to_string()));

        // Test updating non-existent policy
        let not_updated = update_policy_status(&test_db.pool, 99999, "cancelled")
            .await
            .unwrap();
        assert!(!not_updated);
    }

    // ============================================================================
    // POLICY CONDITION TESTS
    // ============================================================================

    #[tokio::test]
    async fn test_create_and_get_policy_conditions() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy first
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Condition Test Policy".to_string(),
            policy_type: "drought".to_string(),
            location_latitude: Decimal::from_str("32.7767").unwrap(),
            location_longitude: Decimal::from_str("-96.7970").unwrap(),
            location_h3_index: None,
            location_name: Some("Dallas".to_string()),
            coverage_amount: Decimal::from_str("3000.00").unwrap(),
            premium_amount: Decimal::from_str("150.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::April, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::September, 30).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: Some("dallas_station".to_string()),
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        // Initially no conditions
        let conditions = get_conditions_by_policy_id(&test_db.pool, policy.id)
            .await
            .unwrap();
        assert_eq!(conditions.len(), 0);

        // Create a condition
        let condition_data = CreatePolicyCondition {
            policy_id: policy.id,
            condition_type: "rainfall".to_string(),
            operator: "<".to_string(),
            threshold_value: Decimal::from_str("5.0").unwrap(),
            measurement_unit: "mm".to_string(),
            measurement_period: "daily".to_string(),
            consecutive_days: Some(7),
        };

        let created_condition = create_policy_condition(&test_db.pool, &condition_data)
            .await
            .unwrap();

        assert_eq!(created_condition.policy_id, policy.id);
        assert_eq!(created_condition.condition_type, "rainfall");
        assert_eq!(created_condition.operator, "<");
        assert_eq!(
            created_condition.threshold_value,
            Decimal::from_str("5.0").unwrap()
        );
        assert_eq!(created_condition.consecutive_days, Some(7));

        // Now should have one condition
        let conditions = get_conditions_by_policy_id(&test_db.pool, policy.id)
            .await
            .unwrap();
        assert_eq!(conditions.len(), 1);
        assert_eq!(conditions[0].id, created_condition.id);
    }

    #[tokio::test]
    async fn test_create_policy_condition_with_defaults() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy first
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Default Condition Test".to_string(),
            policy_type: "temperature".to_string(),
            location_latitude: Decimal::from_str("25.7617").unwrap(),
            location_longitude: Decimal::from_str("-80.1918").unwrap(),
            location_h3_index: None,
            location_name: Some("Miami".to_string()),
            coverage_amount: Decimal::from_str("1200.00").unwrap(),
            premium_amount: Decimal::from_str("60.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::December, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: None,
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        // Create condition without consecutive_days (should default to 1)
        let condition_data = CreatePolicyCondition {
            policy_id: policy.id,
            condition_type: "temperature_min".to_string(),
            operator: "<".to_string(),
            threshold_value: Decimal::from_str("0.0").unwrap(),
            measurement_unit: "celsius".to_string(),
            measurement_period: "daily".to_string(),
            consecutive_days: None, // Should default to 1
        };

        let created_condition = create_policy_condition(&test_db.pool, &condition_data)
            .await
            .unwrap();
        assert_eq!(created_condition.consecutive_days, Some(1)); // Default value
    }

    // ============================================================================
    // WEATHER DATA TESTS
    // ============================================================================

    #[tokio::test]
    async fn test_insert_weather_data() {
        let test_db = create_test_db().await;

        let weather_data = CreateWeatherData {
            station_id: "test_station_001".to_string(),
            recorded_at: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(12, 0, 0).unwrap(),
            ),
            temperature: Some(Decimal::from_str("22.5").unwrap()),
            humidity: Some(Decimal::from_str("65.0").unwrap()),
            precipitation: Some(Decimal::from_str("0.0").unwrap()),
            wind_speed: Some(Decimal::from_str("15.2").unwrap()),
            wind_direction: Some(Decimal::from_str("180.0").unwrap()),
            atmospheric_pressure: Some(Decimal::from_str("1013.25").unwrap()),
            data_source: Some("weatherxm".to_string()),
            raw_data: Some(serde_json::json!({"test": "data"})),
            quality_score: Some(95),
        };

        let inserted_data = insert_weather_data(&test_db.pool, &weather_data)
            .await
            .unwrap();

        assert_eq!(inserted_data.station_id, "test_station_001");
        assert_eq!(
            inserted_data.temperature,
            Some(Decimal::from_str("22.5").unwrap())
        );
        assert_eq!(
            inserted_data.humidity,
            Some(Decimal::from_str("65.0").unwrap())
        );
        assert_eq!(inserted_data.data_source, Some("weatherxm".to_string()));
        assert_eq!(inserted_data.quality_score, Some(95));
        assert!(inserted_data.id > 0);
    }

    #[tokio::test]
    async fn test_insert_weather_data_with_defaults() {
        let test_db = create_test_db().await;

        let weather_data = CreateWeatherData {
            station_id: "test_station_002".to_string(),
            recorded_at: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 16).unwrap(),
                time::Time::from_hms(13, 0, 0).unwrap(),
            ),
            temperature: Some(Decimal::from_str("18.3").unwrap()),
            humidity: None,
            precipitation: Some(Decimal::from_str("2.5").unwrap()),
            wind_speed: None,
            wind_direction: None,
            atmospheric_pressure: None,
            data_source: None, // Should default to "weatherxm"
            raw_data: None,
            quality_score: None,
        };

        let inserted_data = insert_weather_data(&test_db.pool, &weather_data)
            .await
            .unwrap();

        assert_eq!(inserted_data.station_id, "test_station_002");
        assert_eq!(inserted_data.data_source, Some("weatherxm".to_string())); // Default value
        assert_eq!(inserted_data.humidity, None);
        assert_eq!(inserted_data.wind_speed, None);
    }

    #[tokio::test]
    async fn test_weather_data_upsert() {
        let test_db = create_test_db().await;

        let recorded_time = PrimitiveDateTime::new(
            time::Date::from_calendar_date(2024, time::Month::January, 17).unwrap(),
            time::Time::from_hms(14, 0, 0).unwrap(),
        );

        // Insert initial data
        let weather_data = CreateWeatherData {
            station_id: "upsert_station".to_string(),
            recorded_at: recorded_time,
            temperature: Some(Decimal::from_str("20.0").unwrap()),
            humidity: Some(Decimal::from_str("70.0").unwrap()),
            precipitation: Some(Decimal::from_str("0.0").unwrap()),
            wind_speed: None,
            wind_direction: None,
            atmospheric_pressure: None,
            data_source: None,
            raw_data: None,
            quality_score: Some(80),
        };

        let first_insert = insert_weather_data(&test_db.pool, &weather_data)
            .await
            .unwrap();

        // Insert again with same station_id and recorded_at (should update)
        let updated_weather_data = CreateWeatherData {
            station_id: "upsert_station".to_string(),
            recorded_at: recorded_time,
            temperature: Some(Decimal::from_str("25.0").unwrap()), // Changed
            humidity: Some(Decimal::from_str("75.0").unwrap()),    // Changed
            precipitation: Some(Decimal::from_str("1.2").unwrap()), // Changed
            wind_speed: Some(Decimal::from_str("12.0").unwrap()),  // Added
            wind_direction: Some(Decimal::from_str("90.0").unwrap()), // Added
            atmospheric_pressure: Some(Decimal::from_str("1015.0").unwrap()), // Added
            data_source: None,
            raw_data: Some(serde_json::json!({"updated": true})),
            quality_score: Some(90), // Changed
        };

        let upserted_data = insert_weather_data(&test_db.pool, &updated_weather_data)
            .await
            .unwrap();

        // Should be the same record (same ID) but with updated values
        assert_eq!(upserted_data.id, first_insert.id);
        assert_eq!(
            upserted_data.temperature,
            Some(Decimal::from_str("25.0").unwrap())
        );
        assert_eq!(
            upserted_data.humidity,
            Some(Decimal::from_str("75.0").unwrap())
        );
        assert_eq!(
            upserted_data.wind_speed,
            Some(Decimal::from_str("12.0").unwrap())
        );
        assert_eq!(upserted_data.quality_score, Some(90));
    }

    #[tokio::test]
    async fn test_get_weather_data_by_station_and_date_range() {
        let test_db = create_test_db().await;

        let station_id = "range_test_station";

        // Insert multiple weather records
        let dates = [
            (1, 10, 10.0), // January 1
            (1, 15, 15.0), // January 15
            (1, 20, 20.0), // January 20
            (2, 1, 5.0),   // February 1
        ];

        for (month, day, temp) in dates {
            let weather_data = CreateWeatherData {
                station_id: station_id.to_string(),
                recorded_at: PrimitiveDateTime::new(
                    time::Date::from_calendar_date(
                        2024,
                        if month == 1 {
                            time::Month::January
                        } else {
                            time::Month::February
                        },
                        day,
                    )
                    .unwrap(),
                    time::Time::from_hms(12, 0, 0).unwrap(),
                ),
                temperature: Some(Decimal::from_str(&format!("{}", temp)).unwrap()),
                humidity: None,
                precipitation: None,
                wind_speed: None,
                wind_direction: None,
                atmospheric_pressure: None,
                data_source: None,
                raw_data: None,
                quality_score: None,
            };

            insert_weather_data(&test_db.pool, &weather_data)
                .await
                .unwrap();
        }

        // Query for January data only
        let start_date = PrimitiveDateTime::new(
            time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
            time::Time::from_hms(0, 0, 0).unwrap(),
        );
        let end_date = PrimitiveDateTime::new(
            time::Date::from_calendar_date(2024, time::Month::January, 31).unwrap(),
            time::Time::from_hms(23, 59, 59).unwrap(),
        );

        let january_data = get_weather_data_by_station_and_date_range(
            &test_db.pool,
            station_id,
            &start_date,
            &end_date,
        )
        .await
        .unwrap();

        assert_eq!(january_data.len(), 3); // Should get 3 January records

        // Verify they're in chronological order
        assert_eq!(
            january_data[0].temperature,
            Some(Decimal::from_str("10.0").unwrap())
        );
        assert_eq!(
            january_data[1].temperature,
            Some(Decimal::from_str("15.0").unwrap())
        );
        assert_eq!(
            january_data[2].temperature,
            Some(Decimal::from_str("20.0").unwrap())
        );
    }

    // ============================================================================
    // POLICY CLAIM TESTS
    // ============================================================================

    #[tokio::test]
    async fn test_create_and_get_policy_claims() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy first
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Claim Test Policy".to_string(),
            policy_type: "storm".to_string(),
            location_latitude: Decimal::from_str("29.7604").unwrap(),
            location_longitude: Decimal::from_str("-95.3698").unwrap(),
            location_h3_index: None,
            location_name: Some("Houston".to_string()),
            coverage_amount: Decimal::from_str("5000.00").unwrap(),
            premium_amount: Decimal::from_str("250.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::June, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::November, 30).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: Some("houston_station".to_string()),
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        // Initially no claims
        let claims = get_claims_by_policy_id(&test_db.pool, policy.id)
            .await
            .unwrap();
        assert_eq!(claims.len(), 0);

        // Create a claim
        let claim_data = CreatePolicyClaim {
            policy_id: policy.id,
            claim_amount: Decimal::from_str("5000.00").unwrap(),
            trigger_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::August, 15).unwrap(),
                time::Time::from_hms(14, 30, 0).unwrap(),
            ),
            trigger_period_start: Some(PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::August, 15).unwrap(),
                time::Time::from_hms(12, 0, 0).unwrap(),
            )),
            trigger_period_end: Some(PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::August, 15).unwrap(),
                time::Time::from_hms(18, 0, 0).unwrap(),
            )),
            verification_data: Some(serde_json::json!({
                "wind_speed": 85.0,
                "station_id": "houston_station",
                "conditions_met": true
            })),
        };

        let created_claim = create_policy_claim(&test_db.pool, &claim_data)
            .await
            .unwrap();

        assert_eq!(created_claim.policy_id, policy.id);
        assert_eq!(
            created_claim.claim_amount,
            Decimal::from_str("5000.00").unwrap()
        );
        assert_eq!(created_claim.claim_status, Some("pending".to_string())); // Default status
        assert!(created_claim.verification_data.is_some());

        // Now should have one claim
        let claims = get_claims_by_policy_id(&test_db.pool, policy.id)
            .await
            .unwrap();
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].id, created_claim.id);
    }

    #[tokio::test]
    async fn test_update_claim_status_approved() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy and claim first
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Approval Test Policy".to_string(),
            policy_type: "drought".to_string(),
            location_latitude: Decimal::from_str("33.4484").unwrap(),
            location_longitude: Decimal::from_str("-112.0740").unwrap(),
            location_h3_index: None,
            location_name: Some("Phoenix".to_string()),
            coverage_amount: Decimal::from_str("2500.00").unwrap(),
            premium_amount: Decimal::from_str("125.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::May, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::October, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: None,
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        let claim_data = CreatePolicyClaim {
            policy_id: policy.id,
            claim_amount: Decimal::from_str("2500.00").unwrap(),
            trigger_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::July, 20).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            trigger_period_start: None,
            trigger_period_end: None,
            verification_data: Some(serde_json::json!({"drought_days": 14})),
        };

        let claim = create_policy_claim(&test_db.pool, &claim_data)
            .await
            .unwrap();
        assert_eq!(claim.claim_status, Some("pending".to_string()));

        // Approve the claim
        let updated = update_claim_status(&test_db.pool, claim.id, "approved", None)
            .await
            .unwrap();
        assert!(updated);

        // Verify the status and timestamps
        let claims = get_claims_by_policy_id(&test_db.pool, policy.id)
            .await
            .unwrap();
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].claim_status, Some("approved".to_string()));
        assert!(claims[0].approved_at.is_some());
        assert!(claims[0].rejected_at.is_none());
    }

    #[tokio::test]
    async fn test_update_claim_status_rejected() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy and claim first
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Rejection Test Policy".to_string(),
            policy_type: "rain".to_string(),
            location_latitude: Decimal::from_str("47.6062").unwrap(),
            location_longitude: Decimal::from_str("-122.3321").unwrap(),
            location_h3_index: None,
            location_name: Some("Seattle".to_string()),
            coverage_amount: Decimal::from_str("800.00").unwrap(),
            premium_amount: Decimal::from_str("40.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::July, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::July, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: None,
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        let claim_data = CreatePolicyClaim {
            policy_id: policy.id,
            claim_amount: Decimal::from_str("800.00").unwrap(),
            trigger_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::July, 15).unwrap(),
                time::Time::from_hms(16, 0, 0).unwrap(),
            ),
            trigger_period_start: None,
            trigger_period_end: None,
            verification_data: Some(serde_json::json!({"rainfall": 8.0})),
        };

        let claim = create_policy_claim(&test_db.pool, &claim_data)
            .await
            .unwrap();

        // Reject the claim
        let rejection_reason =
            "Rainfall threshold not met according to official weather station data";
        let updated =
            update_claim_status(&test_db.pool, claim.id, "rejected", Some(rejection_reason))
                .await
                .unwrap();
        assert!(updated);

        // Verify the status and rejection details
        let claims = get_claims_by_policy_id(&test_db.pool, policy.id)
            .await
            .unwrap();
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].claim_status, Some("rejected".to_string()));
        assert!(claims[0].rejected_at.is_some());
        assert_eq!(
            claims[0].rejection_reason,
            Some(rejection_reason.to_string())
        );
        assert!(claims[0].approved_at.is_none());
    }

    // ============================================================================
    // COMBINED QUERY TESTS
    // ============================================================================

    #[tokio::test]
    async fn test_get_policy_with_conditions() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Combined Test Policy".to_string(),
            policy_type: "multi".to_string(),
            location_latitude: Decimal::from_str("39.7392").unwrap(),
            location_longitude: Decimal::from_str("-104.9903").unwrap(),
            location_h3_index: None,
            location_name: Some("Denver".to_string()),
            coverage_amount: Decimal::from_str("4000.00").unwrap(),
            premium_amount: Decimal::from_str("200.00").unwrap(),
            currency: None,
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::December, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: None,
            smart_contract_address: None,
            purchase_transaction_hash: None,
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        // Add multiple conditions
        let conditions = vec![
            CreatePolicyCondition {
                policy_id: policy.id,
                condition_type: "rainfall".to_string(),
                operator: "<".to_string(),
                threshold_value: Decimal::from_str("2.0").unwrap(),
                measurement_unit: "mm".to_string(),
                measurement_period: "daily".to_string(),
                consecutive_days: Some(10),
            },
            CreatePolicyCondition {
                policy_id: policy.id,
                condition_type: "temperature_max".to_string(),
                operator: ">".to_string(),
                threshold_value: Decimal::from_str("35.0").unwrap(),
                measurement_unit: "celsius".to_string(),
                measurement_period: "daily".to_string(),
                consecutive_days: Some(3),
            },
        ];

        for condition_data in conditions {
            create_policy_condition(&test_db.pool, &condition_data)
                .await
                .unwrap();
        }

        // Get policy with conditions
        let policy_with_conditions = get_policy_with_conditions(&test_db.pool, policy.id)
            .await
            .unwrap();

        assert!(policy_with_conditions.is_some());
        let policy_with_conditions = policy_with_conditions.unwrap();
        assert_eq!(policy_with_conditions.policy.id, policy.id);
        assert_eq!(policy_with_conditions.conditions.len(), 2);

        // Verify condition details
        let rainfall_condition = policy_with_conditions
            .conditions
            .iter()
            .find(|c| c.condition_type == "rainfall")
            .unwrap();
        assert_eq!(rainfall_condition.operator, "<");
        assert_eq!(rainfall_condition.consecutive_days, Some(10));

        let temp_condition = policy_with_conditions
            .conditions
            .iter()
            .find(|c| c.condition_type == "temperature_max")
            .unwrap();
        assert_eq!(temp_condition.operator, ">");
        assert_eq!(temp_condition.consecutive_days, Some(3));
    }

    #[tokio::test]
    async fn test_get_policy_with_claims() {
        let test_db = create_test_db().await;
        let user_id = create_test_user_for_policies(&test_db.pool).await;

        // Create a policy
        let policy_data = CreateInsurancePolicy {
            user_id,
            policy_template_id: None,
            policy_name: "Claims Test Policy".to_string(),
            policy_type: "comprehensive".to_string(),
            location_latitude: Decimal::from_str("37.7749").unwrap(),
            location_longitude: Decimal::from_str("-122.4194").unwrap(),
            location_h3_index: None,
            location_name: Some("San Francisco".to_string()),
            coverage_amount: Decimal::from_str("10000.00").unwrap(),
            premium_amount: Decimal::from_str("500.00").unwrap(),
            currency: Some("ETH".to_string()),
            start_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            end_date: PrimitiveDateTime::new(
                time::Date::from_calendar_date(2024, time::Month::December, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            ),
            weather_station_id: Some("sf_station".to_string()),
            smart_contract_address: Some("0x742d35Cc6e6B4C4c7d06C3E2b8d7A2b5F7f9e8a4".to_string()),
            purchase_transaction_hash: Some("0x1234567890abcdef".to_string()),
        };

        let policy = create_insurance_policy(&test_db.pool, &policy_data)
            .await
            .unwrap();

        // Add multiple claims
        let claims = vec![
            CreatePolicyClaim {
                policy_id: policy.id,
                claim_amount: Decimal::from_str("3000.00").unwrap(),
                trigger_date: PrimitiveDateTime::new(
                    time::Date::from_calendar_date(2024, time::Month::March, 15).unwrap(),
                    time::Time::from_hms(10, 0, 0).unwrap(),
                ),
                trigger_period_start: None,
                trigger_period_end: None,
                verification_data: Some(serde_json::json!({"event": "drought"})),
            },
            CreatePolicyClaim {
                policy_id: policy.id,
                claim_amount: Decimal::from_str("5000.00").unwrap(),
                trigger_date: PrimitiveDateTime::new(
                    time::Date::from_calendar_date(2024, time::Month::August, 10).unwrap(),
                    time::Time::from_hms(15, 30, 0).unwrap(),
                ),
                trigger_period_start: None,
                trigger_period_end: None,
                verification_data: Some(serde_json::json!({"event": "heat_wave"})),
            },
        ];

        for claim_data in claims {
            create_policy_claim(&test_db.pool, &claim_data)
                .await
                .unwrap();
        }

        // Get policy with claims
        let policy_with_claims = get_policy_with_claims(&test_db.pool, policy.id)
            .await
            .unwrap();

        assert!(policy_with_claims.is_some());
        let policy_with_claims = policy_with_claims.unwrap();
        assert_eq!(policy_with_claims.policy.id, policy.id);
        assert_eq!(policy_with_claims.claims.len(), 2);

        // Verify claims are ordered by creation time (most recent first)
        assert_eq!(
            policy_with_claims.claims[0].claim_amount,
            Decimal::from_str("5000.00").unwrap()
        );
        assert_eq!(
            policy_with_claims.claims[1].claim_amount,
            Decimal::from_str("3000.00").unwrap()
        );
    }

    #[tokio::test]
    async fn test_get_nonexistent_policy_with_conditions() {
        let test_db = create_test_db().await;

        let result = get_policy_with_conditions(&test_db.pool, 99999)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_nonexistent_policy_with_claims() {
        let test_db = create_test_db().await;

        let result = get_policy_with_claims(&test_db.pool, 99999).await.unwrap();
        assert!(result.is_none());
    }
}
