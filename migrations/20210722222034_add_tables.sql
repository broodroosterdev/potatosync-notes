CREATE TABLE IF NOT EXISTS notes
(
    id uuid NOT NULL,
    account_id uuid NOT NULL,
    content bytea NOT NULL,
    last_changed bigint NOT NULL,
    CONSTRAINT notes_pkey PRIMARY KEY (id, account_id)
);

CREATE TABLE IF NOT EXISTS tags
(
    id uuid NOT NULL,
    account_id uuid NOT NULL,
    content bytea NOT NULL,
    last_changed bigint NOT NULL,
    CONSTRAINT tags_pkey PRIMARY KEY (id, account_id)
);


