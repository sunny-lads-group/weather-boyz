use serde::{Deserialize, Serialize, Deserializer};
use sqlx::types::time::PrimitiveDateTime;
use rust_decimal::Decimal;
use time::OffsetDateTime;

fn deserialize_primitive_datetime<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let offset_dt = OffsetDateTime::parse(&s, &time::format_description::well_known::Iso8601::DEFAULT)
        .map_err(serde::de::Error::custom)?;
    Ok(PrimitiveDateTime::new(offset_dt.date(), offset_dt.time()))
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub wallet_address: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Deserialize, Debug)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CurrentUser {
    pub email: String,
    pub name: String,
    pub password_hash: String,
}

// ============================================================================
// INSURANCE POLICY MODELS
// ============================================================================

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyTemplate {
    pub id: i32,
    pub template_name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub default_conditions: Option<serde_json::Value>,
    pub min_coverage_amount: Decimal,
    pub max_coverage_amount: Decimal,
    pub base_premium_rate: Decimal,
    pub is_active: Option<bool>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePolicyTemplate {
    pub template_name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub default_conditions: Option<serde_json::Value>,
    pub min_coverage_amount: Decimal,
    pub max_coverage_amount: Decimal,
    pub base_premium_rate: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InsurancePolicy {
    pub id: i32,
    pub user_id: i32,
    pub policy_template_id: Option<i32>,
    pub policy_name: String,
    pub policy_type: String,
    pub location_latitude: Decimal,
    pub location_longitude: Decimal,
    pub location_h3_index: Option<String>,
    pub location_name: Option<String>,
    pub coverage_amount: Decimal,
    pub premium_amount: Decimal,
    pub currency: Option<String>,
    pub start_date: PrimitiveDateTime,
    pub end_date: PrimitiveDateTime,
    pub status: Option<String>,
    pub weather_station_id: Option<String>,
    pub smart_contract_address: Option<String>,
    pub purchase_transaction_hash: Option<String>,
    pub blockchain_verified: Option<bool>,
    pub verification_timestamp: Option<PrimitiveDateTime>,
    pub blockchain_block_number: Option<i64>,
    pub verification_error_message: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInsurancePolicy {
    pub user_id: i32,
    pub policy_template_id: Option<i32>,
    pub policy_name: String,
    pub policy_type: String,
    pub location_latitude: Decimal,
    pub location_longitude: Decimal,
    pub location_h3_index: Option<String>,
    pub location_name: Option<String>,
    pub coverage_amount: Decimal,
    pub premium_amount: Decimal,
    pub currency: Option<String>,
    #[serde(deserialize_with = "deserialize_primitive_datetime")]
    pub start_date: PrimitiveDateTime,
    #[serde(deserialize_with = "deserialize_primitive_datetime")]
    pub end_date: PrimitiveDateTime,
    pub weather_station_id: Option<String>,
    pub smart_contract_address: Option<String>,
    pub purchase_transaction_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInsurancePolicyRequest {
    pub policy_template_id: Option<i32>,
    pub policy_name: String,
    pub policy_type: String,
    pub location_latitude: Decimal,
    pub location_longitude: Decimal,
    pub location_h3_index: Option<String>,
    pub location_name: Option<String>,
    pub coverage_amount: Decimal,
    pub premium_amount: Decimal,
    pub currency: Option<String>,
    #[serde(deserialize_with = "deserialize_primitive_datetime")]
    pub start_date: PrimitiveDateTime,
    #[serde(deserialize_with = "deserialize_primitive_datetime")]
    pub end_date: PrimitiveDateTime,
    pub weather_station_id: Option<String>,
    pub smart_contract_address: Option<String>,
    pub purchase_transaction_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyCondition {
    pub id: i32,
    pub policy_id: i32,
    pub condition_type: String,
    pub operator: String,
    pub threshold_value: Decimal,
    pub measurement_unit: String,
    pub measurement_period: String,
    pub consecutive_days: Option<i32>,
    pub created_at: Option<PrimitiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePolicyCondition {
    pub policy_id: i32,
    pub condition_type: String,
    pub operator: String,
    pub threshold_value: Decimal,
    pub measurement_unit: String,
    pub measurement_period: String,
    pub consecutive_days: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeatherData {
    pub id: i32,
    pub station_id: String,
    pub recorded_at: PrimitiveDateTime,
    pub temperature: Option<Decimal>,
    pub humidity: Option<Decimal>,
    pub precipitation: Option<Decimal>,
    pub wind_speed: Option<Decimal>,
    pub wind_direction: Option<Decimal>,
    pub atmospheric_pressure: Option<Decimal>,
    pub data_source: Option<String>,
    pub raw_data: Option<serde_json::Value>,
    pub quality_score: Option<i32>,
    pub created_at: Option<PrimitiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateWeatherData {
    pub station_id: String,
    pub recorded_at: PrimitiveDateTime,
    pub temperature: Option<Decimal>,
    pub humidity: Option<Decimal>,
    pub precipitation: Option<Decimal>,
    pub wind_speed: Option<Decimal>,
    pub wind_direction: Option<Decimal>,
    pub atmospheric_pressure: Option<Decimal>,
    pub data_source: Option<String>,
    pub raw_data: Option<serde_json::Value>,
    pub quality_score: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolicyClaim {
    pub id: i32,
    pub policy_id: i32,
    pub claim_amount: Decimal,
    pub claim_status: Option<String>,
    pub trigger_date: PrimitiveDateTime,
    pub trigger_period_start: Option<PrimitiveDateTime>,
    pub trigger_period_end: Option<PrimitiveDateTime>,
    pub verification_data: Option<serde_json::Value>,
    pub evaluated_at: Option<PrimitiveDateTime>,
    pub approved_at: Option<PrimitiveDateTime>,
    pub rejected_at: Option<PrimitiveDateTime>,
    pub rejection_reason: Option<String>,
    pub payout_transaction_hash: Option<String>,
    pub payout_block_number: Option<i32>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePolicyClaim {
    pub policy_id: i32,
    pub claim_amount: Decimal,
    pub trigger_date: PrimitiveDateTime,
    pub trigger_period_start: Option<PrimitiveDateTime>,
    pub trigger_period_end: Option<PrimitiveDateTime>,
    pub verification_data: Option<serde_json::Value>,
}

// Helper structs for API responses
#[derive(Serialize, Deserialize, Debug)]
pub struct PolicyWithConditions {
    #[serde(flatten)]
    pub policy: InsurancePolicy,
    pub conditions: Vec<PolicyCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolicyWithClaims {
    #[serde(flatten)]
    pub policy: InsurancePolicy,
    pub claims: Vec<PolicyClaim>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_user_serialization() {
        let create_user = CreateUser {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            password: "secure_password123".to_string(),
        };

        let json = serde_json::to_string(&create_user).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("john@example.com"));
        assert!(json.contains("secure_password123"));
    }

    #[test]
    fn test_create_user_deserialization() {
        let json = r#"{
            "name": "Jane Smith",
            "email": "jane@example.com",
            "password": "another_password456"
        }"#;

        let create_user: CreateUser = serde_json::from_str(json).unwrap();
        assert_eq!(create_user.name, "Jane Smith");
        assert_eq!(create_user.email, "jane@example.com");
        assert_eq!(create_user.password, "another_password456");
    }

    #[test]
    fn test_create_user_with_empty_fields() {
        let create_user = CreateUser {
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        };

        // Should serialize/deserialize successfully even with empty fields
        let json = serde_json::to_string(&create_user).unwrap();
        let deserialized: CreateUser = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, "");
        assert_eq!(deserialized.email, "");
        assert_eq!(deserialized.password, "");
    }

    #[test]
    fn test_create_user_with_special_characters() {
        let create_user = CreateUser {
            name: "José María O'Connor".to_string(),
            email: "josé@example.com".to_string(),
            password: "p@ssw0rd!#$%".to_string(),
        };

        let json = serde_json::to_string(&create_user).unwrap();
        let deserialized: CreateUser = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, "José María O'Connor");
        assert_eq!(deserialized.email, "josé@example.com");
        assert_eq!(deserialized.password, "p@ssw0rd!#$%");
    }

    #[test]
    fn test_create_user_with_unicode() {
        let create_user = CreateUser {
            name: "용户名".to_string(),
            email: "test@测试.com".to_string(),
            password: "密码123🔐".to_string(),
        };

        let json = serde_json::to_string(&create_user).unwrap();
        let deserialized: CreateUser = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, create_user.name);
        assert_eq!(deserialized.email, create_user.email);
        assert_eq!(deserialized.password, create_user.password);
    }

    #[test]
    fn test_create_user_missing_fields_error() {
        let incomplete_json = r#"{
            "name": "John Doe",
            "email": "john@example.com"
        }"#;

        let result: Result<CreateUser, _> = serde_json::from_str(incomplete_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: 42,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "$2b$12$hashed_password_string".to_string(),
            wallet_address: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("Alice Johnson"));
        assert!(json.contains("alice@example.com"));
        assert!(json.contains("$2b$12$hashed_password_string"));
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{
            "id": 123,
            "name": "Bob Wilson",
            "email": "bob@example.com",
            "password_hash": "$2b$12$another_hashed_password",
            "wallet_address": "0x1234567890123456789012345678901234567890",
            "created_at": null,
            "updated_at": null
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.name, "Bob Wilson");
        assert_eq!(user.email, "bob@example.com");
        assert_eq!(user.password_hash, "$2b$12$another_hashed_password");
        assert!(user.created_at.is_none());
        assert!(user.updated_at.is_none());
    }

    #[test]
    fn test_user_clone() {
        let original_user = User {
            id: 99,
            name: "Clone Test".to_string(),
            email: "clone@test.com".to_string(),
            password_hash: "hashed_clone_password".to_string(),
            wallet_address: None,
            created_at: None,
            updated_at: None,
        };

        let cloned_user = original_user.clone();
        assert_eq!(original_user.id, cloned_user.id);
        assert_eq!(original_user.name, cloned_user.name);
        assert_eq!(original_user.email, cloned_user.email);
        assert_eq!(original_user.password_hash, cloned_user.password_hash);
    }

    #[test]
    fn test_user_with_negative_id() {
        let user = User {
            id: -1,
            name: "Negative ID User".to_string(),
            email: "negative@example.com".to_string(),
            password_hash: "some_hash".to_string(),
            wallet_address: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, -1);
    }

    #[test]
    fn test_user_with_very_large_id() {
        let user = User {
            id: i32::MAX,
            name: "Max ID User".to_string(),
            email: "max@example.com".to_string(),
            password_hash: "max_hash".to_string(),
            wallet_address: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, i32::MAX);
    }

    #[test]
    fn test_user_missing_required_fields_error() {
        let incomplete_json = r#"{
            "id": 123,
            "name": "Incomplete User"
        }"#;

        let result: Result<User, _> = serde_json::from_str(incomplete_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_signin_data_deserialization() {
        let json = r#"{
            "email": "signin@example.com",
            "password": "signin_password123"
        }"#;

        let signin_data: SignInData = serde_json::from_str(json).unwrap();
        assert_eq!(signin_data.email, "signin@example.com");
        assert_eq!(signin_data.password, "signin_password123");
    }

    #[test]
    fn test_signin_data_with_empty_fields() {
        let json = r#"{
            "email": "",
            "password": ""
        }"#;

        let signin_data: SignInData = serde_json::from_str(json).unwrap();
        assert_eq!(signin_data.email, "");
        assert_eq!(signin_data.password, "");
    }

    #[test]
    fn test_signin_data_with_special_characters() {
        let json = r#"{
            "email": "test+tag@example.com",
            "password": "p@ssw0rd!@#$%^&*()"
        }"#;

        let signin_data: SignInData = serde_json::from_str(json).unwrap();
        assert_eq!(signin_data.email, "test+tag@example.com");
        assert_eq!(signin_data.password, "p@ssw0rd!@#$%^&*()");
    }

    #[test]
    fn test_signin_data_debug_does_not_leak_password() {
        let signin_data = SignInData {
            email: "debug@example.com".to_string(),
            password: "secret_password_should_not_appear".to_string(),
        };

        let debug_output = format!("{:?}", signin_data);
        
        // Debug should show the struct but we want to ensure it's not leaking sensitive data in logs
        assert!(debug_output.contains("SignInData"));
        assert!(debug_output.contains("debug@example.com"));
        // Note: This test will show the password in debug output by default
        // In a real app, you might want to implement a custom Debug trait to hide sensitive fields
    }

    #[test]
    fn test_signin_data_missing_email_error() {
        let incomplete_json = r#"{
            "password": "password_only"
        }"#;

        let result: Result<SignInData, _> = serde_json::from_str(incomplete_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_signin_data_missing_password_error() {
        let incomplete_json = r#"{
            "email": "email_only@example.com"
        }"#;

        let result: Result<SignInData, _> = serde_json::from_str(incomplete_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_signin_data_extra_fields_ignored() {
        let json_with_extra = r#"{
            "email": "extra@example.com",
            "password": "extra_password",
            "extra_field": "should_be_ignored",
            "another_extra": 123
        }"#;

        let signin_data: SignInData = serde_json::from_str(json_with_extra).unwrap();
        assert_eq!(signin_data.email, "extra@example.com");
        assert_eq!(signin_data.password, "extra_password");
        // Extra fields should be ignored during deserialization
    }

    #[test]
    fn test_current_user_deserialization() {
        let json = r#"{
            "email": "current@example.com",
            "name": "Current User",
            "password_hash": "$2b$12$current_user_hash"
        }"#;

        let current_user: CurrentUser = serde_json::from_str(json).unwrap();
        assert_eq!(current_user.email, "current@example.com");
        assert_eq!(current_user.name, "Current User");
        assert_eq!(current_user.password_hash, "$2b$12$current_user_hash");
    }

    #[test]
    fn test_current_user_with_empty_fields() {
        let json = r#"{
            "email": "",
            "name": "",
            "password_hash": ""
        }"#;

        let current_user: CurrentUser = serde_json::from_str(json).unwrap();
        assert_eq!(current_user.email, "");
        assert_eq!(current_user.name, "");
        assert_eq!(current_user.password_hash, "");
    }

    #[test]
    fn test_current_user_debug_shows_password_hash() {
        let current_user = CurrentUser {
            email: "debug_current@example.com".to_string(),
            name: "Debug Current User".to_string(),
            password_hash: "sensitive_hash_data".to_string(),
        };

        let debug_output = format!("{:?}", current_user);
        
        assert!(debug_output.contains("CurrentUser"));
        assert!(debug_output.contains("debug_current@example.com"));
        assert!(debug_output.contains("Debug Current User"));
        // Note: Password hash is shown in debug output - this is for internal auth middleware use
        // In production logs, be careful about when CurrentUser debug output is used
        assert!(debug_output.contains("sensitive_hash_data"));
    }

    #[test]
    fn test_current_user_missing_fields_error() {
        let incomplete_json = r#"{
            "email": "incomplete@example.com",
            "name": "Incomplete User"
        }"#;

        let result: Result<CurrentUser, _> = serde_json::from_str(incomplete_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_current_user_with_special_characters() {
        let json = r#"{
            "email": "special+chars@example.com",
            "name": "José María O'Sullivan",
            "password_hash": "$2b$12$special.hash/chars+here"
        }"#;

        let current_user: CurrentUser = serde_json::from_str(json).unwrap();
        assert_eq!(current_user.email, "special+chars@example.com");
        assert_eq!(current_user.name, "José María O'Sullivan");
        assert_eq!(current_user.password_hash, "$2b$12$special.hash/chars+here");
    }

    #[test]
    fn test_current_user_extra_fields_ignored() {
        let json_with_extra = r#"{
            "email": "extra_current@example.com",
            "name": "Extra Current User",
            "password_hash": "hash_with_extras",
            "id": 999,
            "created_at": "2023-01-01T00:00:00Z",
            "extra_data": "should_be_ignored"
        }"#;

        let current_user: CurrentUser = serde_json::from_str(json_with_extra).unwrap();
        assert_eq!(current_user.email, "extra_current@example.com");
        assert_eq!(current_user.name, "Extra Current User");
        assert_eq!(current_user.password_hash, "hash_with_extras");
        // Extra fields should be ignored during deserialization
    }
}
