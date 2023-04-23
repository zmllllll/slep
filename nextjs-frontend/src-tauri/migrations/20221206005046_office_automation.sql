CREATE TABLE user_group (
  id BIGINT NOT NULL,
  pid BIGINT,
  name TEXT NOT NULL,
  des TEXT,
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (id)
);
CREATE INDEX slep_user_group_id_idx ON user_group (id);

CREATE TABLE group_member (
  gid BIGINT NOT NULL,
  uid BIGINT NOT NULL,
  level SMALLINT NOT NULL DEFAULT 0 CHECK(level > 0),
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (uid, gid)
);
CREATE INDEX slep_group_member_gid_idx ON group_member (gid);
CREATE INDEX slep_group_member_uid_idx ON group_member (uid);
CREATE INDEX slep_group_member_uid_gid_idx ON group_member (gid, uid);

-- CREATE TYPE slep.office_automation_task_status AS ENUM ('unknow', 'created', 'pending', 'confirmed', 'blocked', 'failed');

CREATE TABLE office_automation_task_receipt (
  task_id BIGINT NOT NULL,
  executor BIGINT NOT NULL,
  status TEXT NOT NULL,
  des TEXT NOT NULL,
  timestamp BIGINT NOT NULL,
  PRIMARY KEY (task_id, executor, timestamp)
);

-- CREATE TYPE slep.office_automation_task_type AS ENUM ('unknow', 'group', 'private');

CREATE TABLE office_automation_task (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  des TEXT,
  typ TEXT NOT NULL,
  consignor BIGINT NOT NULL,
  deadline BIGINT NOT NULL DEFAULT 0,
  timestamp BIGINT NOT NULL,
--   receipts office_automation_task_receipt[],
  CONSTRAINT slep_office_automation_task_pkey PRIMARY KEY (id)
);
CREATE INDEX slep_office_automation_task_idx ON office_automation_task (id);