CREATE TABLE certifications (
    id             SERIAL      PRIMARY KEY,
    title          TEXT        NOT NULL,
    issuer         TEXT        NOT NULL,
    issued_at      DATE        NOT NULL,
    expired_at     DATE,
    credential_id  TEXT        NOT NULL DEFAULT '',
    credential_url TEXT        NOT NULL DEFAULT '',
    image_src      TEXT        NOT NULL DEFAULT '',
    sort_order     INT         NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
