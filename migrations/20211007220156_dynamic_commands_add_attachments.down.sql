-- Add down migration script here
ALTER TABLE dynamic_commands DROP COLUMN attachment_urls RESTRICT;
