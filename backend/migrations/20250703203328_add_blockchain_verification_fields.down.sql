-- Remove blockchain verification fields from insurance_policies table
ALTER TABLE insurance_policies 
DROP CONSTRAINT IF EXISTS unique_purchase_transaction_hash;

DROP INDEX IF EXISTS idx_insurance_policies_blockchain_verified;

ALTER TABLE insurance_policies 
DROP COLUMN IF EXISTS blockchain_verified,
DROP COLUMN IF EXISTS verification_timestamp,
DROP COLUMN IF EXISTS blockchain_block_number,
DROP COLUMN IF EXISTS verification_error_message;

-- Remove wallet address from users table
ALTER TABLE users
DROP COLUMN IF EXISTS wallet_address;
