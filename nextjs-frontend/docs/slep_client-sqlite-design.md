# Selp Client Sqlite design

Author: qians
Data: 2022-12-6
Version: 0.0.1

## statement

```sql
-- Get UserInfo
SELECT id, name FROM user WHERE uid = $1;

-- Set Message
INSERT INTO message (id, typ, addr_typ, addr, topic, content, sender, receiver, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT (id) DO UPDATE SET typ= excluded.typ, addr_typ= excluded.addr_typ, addr= excluded.addr, topic= excluded.topic, content= excluded.content, sender= excluded.sender, receiver= excluded.receiver, timestamp= excluded.timestamp;

-- Get Private Messages
SELECT id, typ, addr_typ, addr, topic, content, sender, receiver FROM message WHERE addr_typ = 'private' AND (addr = $1 OR sender = $1);

-- Get Stream Messages
SELECT id, typ, addr_typ, addr, topic, content, sender, receiver FROM message WHERE addr_typ = 'stream' AND addr = $1;

-- Get Messages By Addr $ Topic
{get messages} AND topic = $1;

-- Set Message Read Status
INSERT INTO message_read_status (uid, addr_hash, latest_mid) VALUES ($1,$2,$3) ON CONFLICT (uid, addr_hash) DO UPDATE SET latest_mid = excluded.latest_mid;

-- Get Message Read Status
SELECT uid, addr_hash, latest_mid FROM message_read_status WHERE uid = $1;

-- Set StreamSettings
INSERT INTO stream_settings (stream, gid, des, rlevel, wlevel) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (stream, gid) DO UPDATE SET des= excluded.des, rlevel= excluded.rlevel, wlevel= excluded.wlevel;

-- Get StreamSettings
SELECT stream, gid, des, rlevel, wlevel FROM stream_settings WHERE gid = $1;

-- Set TopicSettings
INSERT INTO topic_settings (hashkey, associate_task_id, rlevel, wlevel) VALUES ($1, $2, $3, $4) ON CONFLICT (hashkey) DO UPDATE SET associate_task_id= excluded.associate_task_id, rlevel= excluded.rlevel, wlevel= excluded.wlevel;

-- Get TopicSettings
SELECT hashkey, associate_task_id, rlevel, wlevel FROM topic_settings WHERE hashkey = $1;

-- Get Task List By consignor
SELECT id, name, des, typ, consignor, deadline, timestamp FROM office_automation_task WHERE consignor = $1;

-- Get Task List By Status
SELECT id, name, des, typ, consignor, deadline, timestamp FROM office_automation_task WHERE typ = $1;

-- Set Task
INSERT INTO office_automation_task(id, name, des, typ, consignor, deadline, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT slep_office_automation_task_pkey DO UPDATE SET name= excluded.name, des= excluded.des, typ= excluded.typ, consignor= excluded.consignor, deadline= excluded.deadline, timestamp= excluded.timestamp;

-- Get Task Info
SELECT id, name, des, typ, consignor, deadline, timestamp FROM office_automation_task WHERE id = $1;

-- Add Task Receipt
-- TODO:

-- Get The Group To Which The User Belongs
SELECT gid, uid, level FROM group_member WHERE uid =$1;

-- Get GroupMember List
SELECT gid, uid, level FROM group_member WHERE gid =$1;

-- Get Sub Group
SELECT id, pid, name, des FROM user_group WHERE pid = $1;

-- Set Group
INSERT INTO user_group (id, pid, name, des) VALUES ($1, $2, $3, $4) ON CONFLICT (gid) DO UPDATE SET pid= excluded.pid, name= excluded.name, des= excluded.des;

-- Set GroupMember
INSERT INTO group_member (gid, uid, level, timestamp) VALUES ($1, $2, $3, $4) ON CONFLICT (gid, uid) DO UPDATE SET level= excluded.level;
```
