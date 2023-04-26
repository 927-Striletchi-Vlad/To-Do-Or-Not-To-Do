-- Your SQL goes here
CREATE TABLE users(
  uid varchar(255) NOT NULL PRIMARY KEY,
  name varchar(255) NOT NULL,
  email varchar(255) NOT NULL,
  password varchar(255) NOT NULL,
  created_at timestamp without time zone NOT NULL
);
