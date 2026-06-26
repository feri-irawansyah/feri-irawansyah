-- Profile
CREATE TABLE IF NOT EXISTS profiles (
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    title       TEXT NOT NULL,
    bio         TEXT NOT NULL,
    avatar_url  TEXT,
    github_url  TEXT,
    linkedin_url TEXT,
    website_url  TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Experiences
CREATE TABLE IF NOT EXISTS experiences (
    id          SERIAL PRIMARY KEY,
    company     TEXT NOT NULL,
    role        TEXT NOT NULL,
    started_at  DATE NOT NULL,
    ended_at    DATE,
    description TEXT NOT NULL,
    is_current  BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order  INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Projects
CREATE TABLE IF NOT EXISTS projects (
    id          SERIAL PRIMARY KEY,
    title       TEXT NOT NULL,
    slug        TEXT NOT NULL UNIQUE,
    summary     TEXT NOT NULL,
    description TEXT NOT NULL,
    tech_stack  TEXT[] NOT NULL DEFAULT '{}',
    repo_url    TEXT,
    demo_url    TEXT,
    image_url   TEXT,
    featured    BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order  INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Skills
CREATE TABLE IF NOT EXISTS skills (
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL,
    category   TEXT NOT NULL,
    level      SMALLINT NOT NULL DEFAULT 3 CHECK (level BETWEEN 1 AND 5),
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Contact messages
CREATE TABLE IF NOT EXISTS contact_messages (
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL,
    email      TEXT NOT NULL,
    subject    TEXT NOT NULL,
    body       TEXT NOT NULL,
    read       BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
