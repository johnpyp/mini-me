-- Add up migration script here
CREATE TABLE IF NOT EXISTS dynamic_commands (
  id TEXT PRIMARY KEY NOT NULL,
  guild_id TEXT NOT NULL,
  command TEXT NOT NULL,
  response TEXT NOT NULL
);
