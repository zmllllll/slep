CREATE TABLE slep.user_group (
  id BIGINT NOT NULL,
  pid BIGINT,
  name TEXT NOT NULL,
  des TEXT,
  timestamp BIGINT NOT NULL,
  CONSTRAINT slep_user_group_pkey PRIMARY KEY (id),
  CONSTRAINT slep_user_group_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_user_group_id_idx ON slep.user_group (id);

CREATE TABLE slep.group_member (
  gid BIGINT NOT NULL,
  uid BIGINT NOT NULL,
  level SMALLINT NOT NULL DEFAULT 0 CHECK(level > 0),
  timestamp BIGINT NOT NULL,
  CONSTRAINT slep_group_member_pkey PRIMARY KEY (uid, gid),
  CONSTRAINT slep_group_member_conflict EXCLUDE USING gist (uid WITH =, gid WITH =)
);
CREATE INDEX slep_group_member_gid_idx ON slep.group_member (gid);
CREATE INDEX slep_group_member_uid_idx ON slep.group_member (uid);
CREATE INDEX slep_group_member_uid_gid_idx ON slep.group_member (gid, uid);

CREATE TYPE slep.office_automation_task_status AS ENUM ('unknown', 'created', 'pending', 'confirmed', 'blocked', 'failed');

CREATE TYPE slep.office_automation_task_receipt AS (
  executor BIGINT,
  status slep.office_automation_task_status,
  des TEXT,
  timestamp BIGINT
);
CREATE DOMAIN slep.office_automation_task_receipts AS slep.office_automation_task_receipt[];

CREATE TYPE slep.office_automation_task_type AS ENUM ('unknown', 'group', 'private');

CREATE TABLE slep.office_automation_task (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  des TEXT,
  typ slep.office_automation_task_type NOT NULL,
  consignor BIGINT NOT NULL,
  deadline BIGINT NOT NULL DEFAULT 0,
  timestamp BIGINT NOT NULL,
  receipts slep.office_automation_task_receipts,
  CONSTRAINT slep_office_automation_task_pkey PRIMARY KEY (id),
  CONSTRAINT slep_office_automation_task_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_office_automation_task_idx ON slep.office_automation_task (id);