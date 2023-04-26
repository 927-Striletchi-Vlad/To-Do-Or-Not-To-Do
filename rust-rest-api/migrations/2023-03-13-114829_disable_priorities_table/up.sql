-- Your SQL goes here

ALTER TABLE todolists
DROP CONSTRAINT fk_priority;

DROP TABLE IF EXISTS priorities;
