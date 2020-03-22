-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS notes;
DROP TABLE IF EXISTS tokens;
DROP TABLE IF EXISTS accounts;
DROP SEQUENCE IF EXISTS accounts_id_seq;