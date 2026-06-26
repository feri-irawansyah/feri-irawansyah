CREATE TABLE experience (
    id          SERIAL      PRIMARY KEY,
    position_id INT         NOT NULL REFERENCES positions(id) ON DELETE CASCADE,
    description TEXT        NOT NULL,
    sort_order  INT         NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
