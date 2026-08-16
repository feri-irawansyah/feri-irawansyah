ALTER TABLE users
    ADD COLUMN mfa_secret          TEXT    DEFAULT NULL,
    ADD COLUMN mfa_enabled         BOOLEAN DEFAULT NULL,
    ADD COLUMN mfa_recovery_codes  TEXT[]  DEFAULT NULL;
