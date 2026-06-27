CREATE TABLE certifications (
    id          SERIAL      PRIMARY KEY,
    title       TEXT        NOT NULL,
    url_docs    TEXT        NOT NULL DEFAULT '',
    image_src   TEXT        NOT NULL DEFAULT '',
    description TEXT        NOT NULL DEFAULT '',
    tech        INT4[]      NOT NULL DEFAULT '{}',
    start_date  DATE        NOT NULL,
    last_update TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
