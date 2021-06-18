-- Add up migration script here
CREATE TABLE IF NOT EXISTS guild_data (
  guild_id TEXT PRIMARY KEY NOT NULL,
  moderator_role_id TEXT
);
