-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notes
(
    id              text    not null,
    account_id      text    not null,
    content         text    not null,
    last_changed    bigint  not null,
    PRIMARY KEY (id, account_id)
);

CREATE TABLE IF NOT EXISTS tags
(
    id              text    not null,
    account_id      text    not null,
    content         text    not null,
    last_changed    bigint  not null,
    PRIMARY KEY (id, account_id)
);