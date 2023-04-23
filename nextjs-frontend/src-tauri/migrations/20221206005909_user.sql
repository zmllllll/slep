CREATE TABLE user (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  profile_picture Text,
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (id)
);
CREATE INDEX slep_user_id_idx ON user (id);