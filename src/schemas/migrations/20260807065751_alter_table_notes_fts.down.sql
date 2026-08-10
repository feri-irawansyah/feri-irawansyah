DROP TRIGGER IF EXISTS notes_tsv_update ON notes;

DROP FUNCTION IF EXISTS notes_tsv_update();

DROP INDEX IF EXISTS notes_tsv_idx;

ALTER TABLE notes
DROP COLUMN IF EXISTS tsv;
