-- Add up migration script here
ALTER TABLE projects
DROP COLUMN begin_date,
DROP COLUMN end_date;