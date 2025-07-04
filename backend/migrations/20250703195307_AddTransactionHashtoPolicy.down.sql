-- Add down migration script here
ALTER TABLE insurance_policies
DROP COLUMN transaction_hash;