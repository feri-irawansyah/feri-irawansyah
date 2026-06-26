CREATE TABLE notes (
    notes_id    SERIAL      PRIMARY KEY,
    category    TEXT        NOT NULL,
    title       TEXT        NOT NULL,
    slug        TEXT        NOT NULL UNIQUE,
    content     TEXT        NOT NULL,
    description TEXT        NOT NULL,
    hashtag     TEXT[]      NOT NULL DEFAULT '{}',
    published   BOOLEAN     NOT NULL DEFAULT FALSE,
    ip_address  TEXT        NOT NULL DEFAULT '',
    last_update TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
