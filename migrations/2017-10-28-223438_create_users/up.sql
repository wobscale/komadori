-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  -- note, postgres creates an index since it's UNIQUE
  uuid uuid UNIQUE DEFAULT uuid_generate_v4(),
  username TEXT UNIQUE NOT NULL,
  display_name TEXT,

  role TEXT,

  email TEXT,

  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('users');

-- A user may have many github accounts associated with their account,
-- but each github account may only be associated with one user
CREATE TABLE github_accounts (
  id integer UNIQUE NOT NULL PRIMARY KEY, -- gh account id
  user_id integer REFERENCES users (id),

  access_token TEXT NOT NULL,
  avatar TEXT NOT NULL
);
