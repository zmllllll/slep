CREATE TYPE slep.message_type AS ENUM ('unknown', 'md', 'img', 'video', 'audio', 'bot');
CREATE TABLE slep.message (
  id BIGINT NOT NULL,
  typ slep.message_type NOT NULL,
  gid BIGINT,
  -- private: uid, stream: stream
  addr TEXT NOT NULL,
  topic TEXT NOT NULL,
  content TEXT NOT NULL,
  sender BIGINT NOT NULL,
  timestamp BIGINT NOT NULL,
  CONSTRAINT slep_message_pkey PRIMARY KEY (id),
  CONSTRAINT slep_message_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_message_id_idx ON slep.message (id);
CREATE TABLE slep.stream_settings (
  stream TEXT NOT NULL,
  gid BIGINT NOT NULL,
  des TEXT,
  rlevel SMALLINT NOT NULL DEFAULT 0 CHECK(rlevel >= 0),
  wlevel SMALLINT NOT NULL DEFAULT 0 CHECK(wlevel >= 0),
  timestamp BIGINT NOT NULL,
  CONSTRAINT slep_stream_settings_pkey PRIMARY KEY (stream, gid),
  CONSTRAINT slep_stream_settings_conflict EXCLUDE USING gist (stream WITH =, gid WITH =)
);
CREATE INDEX slep_stream_settings_gid_idx ON slep.stream_settings (gid);
CREATE INDEX slep_stream_settings_stream_gid_idx ON slep.stream_settings (stream, gid);
CREATE TABLE slep.topic_settings (
  hashkey BIGINT NOT NULL,
  associate_task_id BIGINT,
  rlevel SMALLINT NOT NULL DEFAULT 0 CHECK(rlevel >= 0),
  wlevel SMALLINT NOT NULL DEFAULT 0 CHECK(wlevel >= 0),
  timestamp BIGINT NOT NULL,
  CONSTRAINT slep_topic_settings_pkey PRIMARY KEY (hashkey),
  CONSTRAINT slep_topic_settings_conflict EXCLUDE USING gist (hashkey WITH =)
);
CREATE INDEX slep_topic_settings_idx ON slep.topic_settings (hashkey);
CREATE TABLE slep.read_status (
  addr JSONB NOT NULL,
  latest_message_id BIGINT NOT NULL,
  CONSTRAINT slep_read_status_pkey PRIMARY KEY (addr)
);
CREATE INDEX slep_read_status_addr_idx ON slep.read_status USING GIN (addr, (addr->'uid'));