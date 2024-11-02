-- Add up migration script here
ALTER TABLE work_items
DROP COLUMN type;

ALTER TABLE work_items
ADD COLUMN w_type VARCHAR NOT NULL