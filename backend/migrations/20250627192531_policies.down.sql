-- Rollback Weather Insurance Policy Tables
-- Drop tables in reverse order to handle foreign key constraints

-- Drop indexes first
DROP INDEX IF EXISTS idx_policy_templates_active;
DROP INDEX IF EXISTS idx_policy_templates_type;
DROP INDEX IF EXISTS idx_policy_claims_trigger_date;
DROP INDEX IF EXISTS idx_policy_claims_status;
DROP INDEX IF EXISTS idx_policy_claims_policy_id;
DROP INDEX IF EXISTS idx_weather_data_recorded_at;
DROP INDEX IF EXISTS idx_weather_data_station_time;
DROP INDEX IF EXISTS idx_policy_conditions_type;
DROP INDEX IF EXISTS idx_policy_conditions_policy_id;
DROP INDEX IF EXISTS idx_insurance_policies_station;
DROP INDEX IF EXISTS idx_insurance_policies_dates;
DROP INDEX IF EXISTS idx_insurance_policies_location;
DROP INDEX IF EXISTS idx_insurance_policies_status;
DROP INDEX IF EXISTS idx_insurance_policies_user_id;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS policy_claims;
DROP TABLE IF EXISTS weather_data;
DROP TABLE IF EXISTS policy_conditions;
DROP TABLE IF EXISTS insurance_policies;
DROP TABLE IF EXISTS policy_templates;
