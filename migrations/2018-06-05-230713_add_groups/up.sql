-- Your SQL goes here

CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  uuid uuid UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  public BOOLEAN NOT NULL DEFAULT true,

  description TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('groups');

CREATE TABLE users_groups (
  user_id integer NOT NULL REFERENCES users (id),
  group_id integer NOT NULL REFERENCES groups (id),

  owner BOOLEAN NOT NULL DEFAULT false, -- is a user an owner of a group?

  CONSTRAINT users_groups_pkey PRIMARY KEY (user_id, group_id)

);
