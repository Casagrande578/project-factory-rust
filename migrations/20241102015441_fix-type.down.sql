-- Add down migration script here
ALTER TABLE work_items
DROP COLUMN w_type;

ALTER TABLE work_items
ADD COLUMN type VARCHAR NOT NULL