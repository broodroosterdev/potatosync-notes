-- Your SQL goes here
CREATE SEQUENCE accounts_id_seq;

CREATE TABLE IF NOT EXISTS accounts
(
    id                  integer unique primary key default nextval('accounts_id_seq'),
    created_at          text    not null,
    updated_at          text,
    deleted_at          text,
    email               text    not null,
    username            text    not null,
    password            text    not null,
    image_url           text    not null,
    password_identifier text    not null,
    verified            boolean not null
);

CREATE TABLE IF NOT EXISTS notes
(
    note_id           integer not null,
    account_id        integer not null REFERENCES accounts (id),
    title             text    not null,
    content           text    not null,
    image_url         text,
    list_parse_string text,
    reminders         text,
    date              text    not null,
    color             integer not null,
    hide_content      boolean not null,
    is_deleted        boolean not null,
    is_archived       boolean not null,
    is_list           boolean not null,
    is_starred        boolean not null,
    pin               text,
    password          text,
    PRIMARY KEY (note_id, account_id)
);

CREATE INDEX IF NOT EXISTS idx_accounts_deleted_at ON accounts (deleted_at);

CREATE TABLE IF NOT EXISTS tokens
(
    account_id integer not null PRIMARY KEY REFERENCES accounts (id),
    token      text    not null
)