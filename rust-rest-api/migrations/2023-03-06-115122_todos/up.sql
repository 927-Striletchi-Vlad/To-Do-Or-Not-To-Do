-- Your SQL goes here
CREATE TABLE todos (
  tid character varying(255) PRIMARY KEY,
  title character varying(255) NOT NULL,
  content character varying(255) NOT NULL,
  completed boolean NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone

);
