CREATE TABLE skills (
    skill_id    SERIAL      PRIMARY KEY,
    title       TEXT        NOT NULL,
    description TEXT        NOT NULL DEFAULT '',
    url_docs    TEXT        NOT NULL DEFAULT '',
    image_src   TEXT        NOT NULL DEFAULT '',
    progress    INT         NOT NULL DEFAULT 0 CHECK (progress BETWEEN 0 AND 100),
    star        INT         NOT NULL DEFAULT 0 CHECK (star BETWEEN 0 AND 5),
    experience  INT         NOT NULL DEFAULT 0,
    tech_category TEXT        NOT NULL DEFAULT '',
    last_update TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
