-- Add migration script here
-- add to users table
ALTER TABLE users
ADD COLUMN avatar_url text,
ADD COLUMN github_id text,
ADD COLUMN github_url text;
