CREATE EXTENSION IF NOT EXISTS "citext";

DO $$ BEGIN
    CREATE TYPE user_status AS ENUM ('unverified', 'verified', 'locked', 'disabled');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username CITEXT UNIQUE NOT NULL,
    email CITEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    status user_status NOT NULL DEFAULT 'unverified',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE refresh_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash CHAR(64) UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    version BIGINT NOT NULL DEFAULT 1
);
