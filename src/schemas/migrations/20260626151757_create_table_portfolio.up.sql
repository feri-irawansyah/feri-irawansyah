CREATE TABLE portfolio (
    portfolio_id SERIAL      PRIMARY KEY,
    title        TEXT        NOT NULL,
    slug         TEXT        NOT NULL UNIQUE,
    description  TEXT        NOT NULL,
    url_docs     TEXT        NOT NULL DEFAULT '',
    image_src    TEXT        NOT NULL DEFAULT '',
    tech         INT4[]      NOT NULL DEFAULT '{}',
    featured     BOOLEAN     NOT NULL DEFAULT FALSE,
    sort_order   INT         NOT NULL DEFAULT 0,
    last_update  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
