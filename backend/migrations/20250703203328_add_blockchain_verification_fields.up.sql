-- Add blockchain verification fields to insurance_policies table
ALTER TABLE insurance_policies 
ADD COLUMN blockchain_verified BOOLEAN DEFAULT false,
ADD COLUMN verification_timestamp TIMESTAMP NULL,
ADD COLUMN blockchain_block_number BIGINT NULL,
ADD COLUMN verification_error_message TEXT NULL;

-- Add wallet address to users table
ALTER TABLE users
ADD COLUMN wallet_address VARCHAR(42) NULL;

-- Create index for efficient queries on verification status
CREATE INDEX idx_insurance_policies_blockchain_verified ON insurance_policies(blockchain_verified);

-- Create unique constraint to prevent duplicate transaction hashes
ALTER TABLE insurance_policies 
ADD CONSTRAINT unique_purchase_transaction_hash 
UNIQUE (purchase_transaction_hash);

-- Update existing policies to be marked as verified for backward compatibility
UPDATE insurance_policies 
SET blockchain_verified = true, verification_timestamp = created_at 
WHERE purchase_transaction_hash IS NOT NULL;
