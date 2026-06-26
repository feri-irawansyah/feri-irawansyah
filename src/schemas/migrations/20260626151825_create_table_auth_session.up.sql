CREATE TABLE auth_sessions (
    id          SERIAL      PRIMARY KEY,
    user_id     INT         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token       TEXT        NOT NULL UNIQUE,
    app_name    TEXT        NOT NULL DEFAULT '',
    device_name TEXT        NOT NULL DEFAULT '',
    ip_address  TEXT        NOT NULL DEFAULT '',
    expired_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
