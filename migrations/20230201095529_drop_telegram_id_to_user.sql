-- Add migration script here
ALTER TABLE users ALTER COLUMN telegram_id TYPE varchar(12);
ALTER TABLE users ALTER COLUMN telegram_id DROP NOT NULL;
