-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS accounts_id_seq;

CREATE TABLE IF NOT EXISTS accounts
(
    id                  text primary key,
    created_at          timestamptz not null,
    updated_at          timestamptz,
    deleted_at          timestamptz,
    email               text        not null,
    username            text        not null,
    password            text        not null,
    image_url           text        not null,
    password_identifier text        not null,
    verified            boolean     not null,
    shared_prefs        text        not null
);

CREATE TABLE IF NOT EXISTS notes
(
    note_id          text        not null,
    account_id       text        not null REFERENCES accounts (id),
    title            text        not null,
    content          text        not null,
    style_json       text        not null,
    starred          bool        not null,
    creation_date    timestamptz not null,
    last_modify_date timestamptz not null,
    color            integer     not null,
    images           text        not null,
    list             bool        not null,
    list_content     text        not null,
    reminders        text        not null,
    hide_content     bool        not null,
    lock_note        bool        not null,
    uses_biometrics  bool        not null,
    deleted          bool        not null,
    archived         bool        not null,
    synced           bool        not null,
    PRIMARY KEY (note_id, account_id)
);

CREATE INDEX IF NOT EXISTS idx_accounts_deleted_at ON accounts (deleted_at);

CREATE TABLE IF NOT EXISTS tokens
(
    account_id text not null PRIMARY KEY REFERENCES accounts (id),
    token      text not null,
    created_at text not null
);