-- Your SQL goes here
CREATE TABLE todostodolists(
  tid varchar(255) NOT NULL,
  tlid varchar(255) NOT NULL,
  PRIMARY KEY (tlid, tid),
  CONSTRAINT fk_tlid
    FOREIGN KEY (tlid)
      REFERENCES todolists(tlid),
  CONSTRAINT fk_tid
    FOREIGN KEY (tid)
      REFERENCES todos(tid)
);
