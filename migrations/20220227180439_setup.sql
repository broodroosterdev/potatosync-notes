-- Add migration script here
CREATE TABLE IF NOT EXISTS blob
(
    id uuid NOT NULL,
    account_id uuid NOT NULL,
    blob_type text NOT NULL,
    content bytea NOT NULL,
    last_changed bigint NOT NULL,
    CONSTRAINT blob_pkey PRIMARY KEY (id, account_id, blob_type)
);