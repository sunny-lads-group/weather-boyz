-- Weather Insurance Policy Tables

-- Policy Templates: Pre-defined insurance product catalog
CREATE TABLE policy_templates (
    id SERIAL PRIMARY KEY,
    template_name VARCHAR(255) NOT NULL,
    description TEXT,
    policy_type VARCHAR(100) NOT NULL, -- 'drought', 'rain', 'temperature', 'wind', 'storm'
    default_conditions JSONB, -- Default trigger conditions for this template
    min_coverage_amount DECIMAL(15,2) NOT NULL,
    max_coverage_amount DECIMAL(15,2) NOT NULL,
    base_premium_rate DECIMAL(8,4) NOT NULL, -- Base premium as percentage or flat rate
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Main Insurance Policies Table
CREATE TABLE insurance_policies (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    policy_template_id INTEGER REFERENCES policy_templates(id),
    policy_name VARCHAR(255) NOT NULL,
    policy_type VARCHAR(100) NOT NULL,
    
    -- Location information
    location_latitude DECIMAL(10,8) NOT NULL,
    location_longitude DECIMAL(11,8) NOT NULL,
    location_h3_index VARCHAR(20), -- For efficient geospatial queries
    location_name VARCHAR(255),
    
    -- Financial details
    coverage_amount DECIMAL(15,2) NOT NULL,
    premium_amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'ETH',
    
    -- Policy period
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    
    -- Status and metadata
    status VARCHAR(50) DEFAULT 'active', -- 'active', 'expired', 'claimed', 'cancelled'
    weather_station_id VARCHAR(100), -- WeatherXM station ID
    
    -- Blockchain integration
    smart_contract_address VARCHAR(42), -- Ethereum contract address
    purchase_transaction_hash VARCHAR(66), -- Transaction hash for policy purchase
    
    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Policy Conditions: Weather trigger conditions for each policy
CREATE TABLE policy_conditions (
    id SERIAL PRIMARY KEY,
    policy_id INTEGER NOT NULL REFERENCES insurance_policies(id) ON DELETE CASCADE,
    condition_type VARCHAR(100) NOT NULL, -- 'rainfall', 'temperature_min', 'temperature_max', 'wind_speed', etc.
    operator VARCHAR(10) NOT NULL, -- '>', '<', '>=', '<=', '==', '!='
    threshold_value DECIMAL(10,4) NOT NULL,
    measurement_unit VARCHAR(20) NOT NULL, -- 'mm', 'celsius', 'fahrenheit', 'km/h', 'mph'
    measurement_period VARCHAR(50) NOT NULL, -- 'daily', 'weekly', 'monthly', 'cumulative'
    consecutive_days INTEGER DEFAULT 1, -- How many consecutive days condition must be met
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Weather Data: Cache of WeatherXM API data
CREATE TABLE weather_data (
    id SERIAL PRIMARY KEY,
    station_id VARCHAR(100) NOT NULL,
    recorded_at TIMESTAMP NOT NULL,
    
    -- Core weather measurements
    temperature DECIMAL(6,2), -- Celsius
    humidity DECIMAL(5,2), -- Percentage
    precipitation DECIMAL(8,2), -- mm
    wind_speed DECIMAL(6,2), -- km/h
    wind_direction DECIMAL(5,2), -- degrees
    atmospheric_pressure DECIMAL(8,2), -- hPa
    
    -- Metadata
    data_source VARCHAR(50) DEFAULT 'weatherxm',
    raw_data JSONB, -- Full API response for debugging
    quality_score INTEGER, -- Data quality indicator
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure unique records per station per time
    UNIQUE(station_id, recorded_at)
);

-- Policy Claims: Insurance claim records and payouts
CREATE TABLE policy_claims (
    id SERIAL PRIMARY KEY,
    policy_id INTEGER NOT NULL REFERENCES insurance_policies(id) ON DELETE CASCADE,
    claim_amount DECIMAL(15,2) NOT NULL,
    claim_status VARCHAR(50) DEFAULT 'pending', -- 'pending', 'approved', 'rejected', 'paid'
    
    -- Trigger information
    trigger_date TIMESTAMP NOT NULL, -- When weather conditions were met
    trigger_period_start TIMESTAMP, -- Start of period that triggered claim
    trigger_period_end TIMESTAMP, -- End of period that triggered claim
    verification_data JSONB, -- Weather data that triggered the claim
    
    -- Processing information
    evaluated_at TIMESTAMP, -- When claim was evaluated
    approved_at TIMESTAMP, -- When claim was approved
    rejected_at TIMESTAMP, -- When claim was rejected
    rejection_reason TEXT, -- Reason for rejection
    
    -- Blockchain payout
    payout_transaction_hash VARCHAR(66), -- Transaction hash of payout
    payout_block_number INTEGER, -- Block number of payout
    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for performance
CREATE INDEX idx_insurance_policies_user_id ON insurance_policies(user_id);
CREATE INDEX idx_insurance_policies_status ON insurance_policies(status);
CREATE INDEX idx_insurance_policies_location ON insurance_policies(location_latitude, location_longitude);
CREATE INDEX idx_insurance_policies_dates ON insurance_policies(start_date, end_date);
CREATE INDEX idx_insurance_policies_station ON insurance_policies(weather_station_id);

CREATE INDEX idx_policy_conditions_policy_id ON policy_conditions(policy_id);
CREATE INDEX idx_policy_conditions_type ON policy_conditions(condition_type);

CREATE INDEX idx_weather_data_station_time ON weather_data(station_id, recorded_at);
CREATE INDEX idx_weather_data_recorded_at ON weather_data(recorded_at);

CREATE INDEX idx_policy_claims_policy_id ON policy_claims(policy_id);
CREATE INDEX idx_policy_claims_status ON policy_claims(claim_status);
CREATE INDEX idx_policy_claims_trigger_date ON policy_claims(trigger_date);

CREATE INDEX idx_policy_templates_type ON policy_templates(policy_type);
CREATE INDEX idx_policy_templates_active ON policy_templates(is_active);

-- Add some sample policy templates
INSERT INTO policy_templates (template_name, description, policy_type, default_conditions, min_coverage_amount, max_coverage_amount, base_premium_rate) VALUES
('Drought Protection', 'Protection against extended periods without rainfall', 'drought', 
 '{"conditions": [{"type": "rainfall", "operator": "<", "threshold": 5, "unit": "mm", "period": "daily", "consecutive_days": 10}]}',
 1.00, 100.00, 0.05),

('Rain Event Insurance', 'Coverage for events that may be cancelled due to rain', 'rain',
 '{"conditions": [{"type": "rainfall", "operator": ">", "threshold": 10, "unit": "mm", "period": "daily", "consecutive_days": 1}]}',
 0.50, 5.00, 0.03),

('Freeze Protection', 'Protection against crop damage from freezing temperatures', 'temperature',
 '{"conditions": [{"type": "temperature_min", "operator": "<", "threshold": -2, "unit": "celsius", "period": "daily", "consecutive_days": 1}]}',
 2.00, 200.00, 0.08),

('Storm Insurance', 'Coverage for damage from high winds and storms', 'wind',
 '{"conditions": [{"type": "wind_speed", "operator": ">", "threshold": 60, "unit": "km/h", "period": "daily", "consecutive_days": 1}]}',
 5.00, 500.00, 0.10);
