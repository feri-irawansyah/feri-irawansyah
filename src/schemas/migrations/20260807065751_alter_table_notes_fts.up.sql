-- The old `tsv` column was leftover from a previous stack: no GIN index, no
-- trigger keeping it in sync, and no repository code ever wrote to it (every
-- note created/edited through this app left it NULL). Drop it and recreate
-- as a generated column so it can never drift from title/description/
-- hashtag/category again — no trigger or Rust-side write path needed.
--
-- Note body text (`content`) is deliberately excluded: that column holds a
-- pointer to a GitHub-hosted markdown file, fetched and rendered at request
-- time, not the article text itself — there is nothing to index there.
ALTER TABLE notes DROP COLUMN tsv;

ALTER TABLE notes ADD COLUMN tsv tsvector
    GENERATED ALWAYS AS (
        setweight(to_tsvector('simple', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('simple', coalesce(description, '')), 'B') ||
        setweight(to_tsvector('simple', coalesce(array_to_string(hashtag, ' '), '')), 'C') ||
        setweight(to_tsvector('simple', coalesce(category, '')), 'D')
    ) STORED;

CREATE INDEX notes_tsv_idx ON notes USING GIN (tsv);
