-- Add down migration script here
ALTER TABLE guild_data DROP COLUMN dynamic_prefix RESTRICT;
