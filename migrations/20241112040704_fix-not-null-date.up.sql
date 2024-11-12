-- Add up migration script here
ALTER TABLE work_items
ALTER COLUMN created_date DROP NOT NULL;