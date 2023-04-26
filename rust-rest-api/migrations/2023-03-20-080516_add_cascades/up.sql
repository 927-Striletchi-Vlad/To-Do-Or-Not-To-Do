-- Your SQL goes here
-- ALTER TABLE todolists
--DROP CONSTRAINT fk_priority;

-- ALTER TABLE todolists
-- ADD CONSTRAINT fk_priority
--   FOREIGN KEY (priority)
--     REFERENCES priorities (pid) ON DELETE CASCADE;

ALTER TABLE todolists
DROP CONSTRAINT fk_uid;

ALTER TABLE todolists
ADD CONSTRAINT fk_uid
  FOREIGN KEY (uid)
    REFERENCES users (uid) ON DELETE CASCADE;

ALTER TABLE todostodolists
DROP CONSTRAINT fk_tlid;

ALTER TABLE todostodolists
ADD CONSTRAINT fk_tlid
  FOREIGN KEY (tlid)
    REFERENCES todolists (tlid) ON DELETE CASCADE;

ALTER TABLE todostodolists
DROP CONSTRAINT fk_tid;

ALTER TABLE todostodolists
ADD CONSTRAINT fk_tid
  FOREIGN KEY (tid)
    REFERENCES todos (tid) ON DELETE CASCADE;


