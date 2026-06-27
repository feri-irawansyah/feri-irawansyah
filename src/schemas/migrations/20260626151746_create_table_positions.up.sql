CREATE TABLE positions (
    id            SERIAL      PRIMARY KEY,
    experience_id INT         NOT NULL,
    title         TEXT        NOT NULL,
    start_date    DATE        NOT NULL,
    end_date      DATE,
    description   TEXT[]      NOT NULL DEFAULT '{}',
    address       TEXT        NOT NULL DEFAULT '',
    job_position  TEXT        NOT NULL DEFAULT '',
    job_type      TEXT        NOT NULL DEFAULT '',
    sort_order    INT         NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
