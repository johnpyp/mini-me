-- Add up migration script here
ALTER TABLE dynamic_commands ADD COLUMN attachment_urls TEXT[];
