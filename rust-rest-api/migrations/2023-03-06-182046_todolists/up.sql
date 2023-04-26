-- Your SQL goes here
CREATE TABLE todolists(
  tlid varchar(255) NOT NULL PRIMARY KEY,
  uid character varying(255) NOT NULL,
  title varchar(255) NOT NULL,
  priority int NOT NULL,
  created_at timestamp  without time zone NOT NULL,

  CONSTRAINT fk_priority
    FOREIGN KEY (priority)
      REFERENCES priorities(pid),

  CONSTRAINT fk_uid
    FOREIGN KEY (uid)
      REFERENCES users(uid)
);
