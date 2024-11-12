-- Add down migration script here
ALTER TABLE work_items
ALTER COLUMN created_date SET NOT NULL;