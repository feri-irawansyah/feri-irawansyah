CREATE TABLE experience (
    id          SERIAL      PRIMARY KEY,
    title       TEXT        NOT NULL,
    company     TEXT        NOT NULL,
    url_docs    TEXT        NOT NULL DEFAULT '',
    image_src   TEXT        NOT NULL DEFAULT '',
    start_date  DATE        NOT NULL,
    end_date    DATE,
    last_update TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

