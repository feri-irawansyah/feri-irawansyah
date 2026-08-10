-- 1. Hapus generated column yang sebelumnya gagal / sudah ada
ALTER TABLE notes
DROP COLUMN IF EXISTS tsv;

-- 2. Buat kolom biasa
ALTER TABLE notes
ADD COLUMN tsv tsvector;

-- 3. Function untuk generate tsvector
CREATE OR REPLACE FUNCTION notes_tsv_update()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    NEW.tsv :=
        setweight(to_tsvector('simple', coalesce(NEW.title, '')), 'A') ||
        setweight(to_tsvector('simple', coalesce(NEW.description, '')), 'B') ||
        setweight(
            to_tsvector(
                'simple',
                coalesce(array_to_string(NEW.hashtag, ' '), '')
            ),
            'C'
        ) ||
        setweight(to_tsvector('simple', coalesce(NEW.category, '')), 'D');

    RETURN NEW;
END;
$$;

-- 4. Trigger
CREATE TRIGGER notes_tsv_update
BEFORE INSERT OR UPDATE OF title, description, hashtag, category
ON notes
FOR EACH ROW
EXECUTE FUNCTION notes_tsv_update();

-- 5. Index
CREATE INDEX notes_tsv_idx
ON notes USING GIN (tsv);

-- 6. Isi tsv untuk data lama
UPDATE notes
SET title = title;
