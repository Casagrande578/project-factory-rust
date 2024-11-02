-- Add down migration script here
ALTER TABLE projects
ADD COLUMN begin_date TIMESTAMP,
ADD COLUMN end_date TIMESTAMP;