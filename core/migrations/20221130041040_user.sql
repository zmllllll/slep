CREATE TABLE slep.user (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  timestamp BIGINT NOT NULL,
  profile_picture TEXT,
  CONSTRAINT slep_user_pkey PRIMARY KEY (id),
  CONSTRAINT slep_user_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_user_id_idx ON slep.user (id);