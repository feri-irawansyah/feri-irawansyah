CREATE TABLE positions (
    id              SERIAL      PRIMARY KEY,
    company         TEXT        NOT NULL,
    role            TEXT        NOT NULL,
    location        TEXT        NOT NULL DEFAULT '',
    employment_type TEXT        NOT NULL DEFAULT 'full-time',
    started_at      DATE        NOT NULL,
    ended_at        DATE,
    is_current      BOOLEAN     NOT NULL DEFAULT FALSE,
    description     TEXT        NOT NULL DEFAULT '',
    sort_order      INT         NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
