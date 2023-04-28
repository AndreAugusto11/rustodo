CREATE TABLE todos (
  id serial NOT NULL,
  title character varying (255) NOT NULL,
  description text NOT NULL,
  completed boolean NOT NULL,
  user_id serial REFERENCES users(id),
  CONSTRAINT todos_pkey PRIMARY KEY (id)
);