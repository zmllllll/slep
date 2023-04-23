CREATE TABLE message (
  id BIGINT NOT NULL,
  gid BIGINT,
  typ TEXT NOT NULL,
  -- private: uid, stream: hash("[gid]stream")
  addr TEXT NOT NULL,
  topic TEXT NOT NULL,
  content TEXT NOT NULL,
  sender BIGINT NOT NULL,
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (id)
);
CREATE INDEX slep_message_id_idx ON message (id);
CREATE TABLE topic_settings (
  hashkey BIGINT NOT NULL,
  associate_task_id BIGINT,
  rlevel SMALLINT NOT NULL DEFAULT 0 CHECK(rlevel >= 0),
  wlevel SMALLINT NOT NULL DEFAULT 0 CHECK(wlevel >= 0),
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (hashkey)
);
CREATE INDEX slep_topic_settings_idx ON topic_settings (hashkey);
CREATE TABLE stream_settings (
  stream TEXT NOT NULL,
  gid BIGINT NOT NULL,
  des TEXT,
  rlevel SMALLINT NOT NULL DEFAULT 0 CHECK(rlevel >= 0),
  wlevel SMALLINT NOT NULL DEFAULT 0 CHECK(wlevel >= 0),
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (stream, gid)
);
CREATE INDEX slep_stream_settings_gid_idx ON stream_settings (gid);
CREATE INDEX slep_stream_settings_stream_gid_idx ON stream_settings (stream, gid);
CREATE TABLE read_status (
  addr TEXT NOT NULL,
  latest_message_id BIGINT NOT NULL,
  PRIMARY KEY (addr)
);
CREATE INDEX slep_read_status_uid_idx ON read_status (addr);