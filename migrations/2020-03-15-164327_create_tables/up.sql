-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notes
(
    note_id          text        not null,
    account_id       text        not null,
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
    PRIMARY KEY (note_id, account_id)
);