ALTER TABLE skills         DROP COLUMN IF EXISTS status;
ALTER TABLE positions      DROP COLUMN IF EXISTS status;
ALTER TABLE portfolio      DROP COLUMN IF EXISTS status;
ALTER TABLE certifications DROP COLUMN IF EXISTS status;
ALTER TABLE experience     DROP COLUMN IF EXISTS status;
