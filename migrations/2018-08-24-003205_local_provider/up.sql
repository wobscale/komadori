-- A user may have many local accounts associated with their account,
-- but each local account may only be associated with one user
CREATE TABLE local_accounts (
  id integer UNIQUE NOT NULL PRIMARY KEY, -- local id
  user_id integer NOT NULL REFERENCES users (id)
);
