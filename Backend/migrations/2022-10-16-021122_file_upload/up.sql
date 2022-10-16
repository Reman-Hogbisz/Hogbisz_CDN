-- Your SQL goes here

CREATE TABLE file_uploads (
    id SERIAL PRIMARY KEY NOT NULL,
    secret VARCHAR(64) NOT NULL,
    name text NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    deleted_at timestamp
);

CREATE TABLE "sessions" (
    id SERIAL PRIMARY KEY NOT NULL,
    session_id VARCHAR(255) NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    expires_at timestamp NOT NULL DEFAULT NOW() + INTERVAL '6 hours'
);