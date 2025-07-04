-- Add up migration script here
ALTER TABLE insurance_policies
ADD COLUMN transaction_hash VARCHAR(255);