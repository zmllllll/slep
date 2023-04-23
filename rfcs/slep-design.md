# Slep Design Document

Author: qians
Data: 2022-11-17
Version: 0.0.1

## Background

在完成氢信后, 对即时通讯有了最基础的认知. 但我认为氢信过于复杂的设计导致了用户的使用体验不好. 用户使用过程中很容易不理解设计者的思路, 再加上引导和工程学做的不好, 导致很多本来寄予重望的功能和设计没有得到正确和良好的使用. 所以在后期我对市面上的一些有代表性的即时通讯产品做了一些调研并深入使用了一段时间, 包括zulip, 飞书, 甚至discord和reddit. 其中zulip的设计最让我惊艳, 我认为zulip的这种Stream和Topic的设计确实真正达到了重新梳理信息流, 减少信噪比的效果. 再加上随着公司开发团队的扩大和项目的增加, 项目管理和OA也被提上日程. 所以在此基础上我希望吸取氢信的经验和教训, 吸收来自zulip和飞书的思想个设计, 并结合项目管理和OA的需求, 设计一款新的即时通讯软件.

## Vision

有用有序, 简单易用, 符合直觉

## Design

整体是在zulip基础上将Topic和项目节点挂钩, 并将user划分出group, 使之更加贴合企业协作和项目管理的需求.

1. 即时通讯部分分两个大模块, Private(私聊)和Stream(群聊)
2. 移除群成员的概念. Group(用户组)和Stream剥离, Group和企业组织架构挂钩.
3. 细化群的概念, 分解成Stream(Private)和Topic. 一个Stream(Private)可以拥有多个Topic.
4. Stream和项目挂钩, Topic用于讨论.
5. 简化Stream和Topic的创建流程, 随用随建(隐式创建).
6. Topic可通过OA系统的任务自动创建. 详细信息和设置来自OA.
7. 对个人的任务派发到Private(私聊), 对项目的任务派发到Stream.
8. 消息体支持Markdown格式, 消息中的URL根据OG(The Open Graph protocol)协议抓取显示

### Group

Group是组织结构的抽象, 大体呈现树状多级结构, 通过父组织字段串联 ``parent_id``

```rust
struct Group {
    id: i64,
    // 父组ID
    pid: i64,
    name: String,
    desc: String,
}
```

组织成员区分权限, 上级组织默认拥有下级组织权限

```rust
struct GroupMember {
    uid: i64,
    gid: i64,
    level: u8,
}
```

### Stream

Stream是消息流的抽象, 一个Stream可设置对某个Group可见, 上级Group默认拥有下级Group的可见性, 但可通过设置scope字段来修改. Group成员可见性跟随当前Group可见性 ``GroupScope``.
成员创建新Stream时所属的Group, 默认拥有此Stream的可见性.

```rust
struct StreamSettings {
    stream: String,
    // 所属的Group
    gid,
    // 描述
    desc: String,
    read_level: u8,
    write_level: u8,
}
```

#### Topic

Topic是对Stream中消息的再分类, 一个Stream可以拥有无数个Topic. Topic可见性跟随所属Stream的可见性.

Topic可选额外设置. 来自OA系统的Topic创建时携带相关数据.

```rust
struct TopicSettings {
    // hash("Gid+Stream+Topic")
    hashkey: i64,
    // 关联OA任务
    associate_task: i64,
    read_level: u8,
    write_level: u8,
}
```

#### OfficeAutomation

OA模块目前主要实现任务的派发和审核, 生成名为 ``name[id]``.若派发给Group, 则生成对应Stream. 派发给个人或Stream则生成Topic.

```rust
enum Delegate{
    Private{
        uid: i64,
        topic: String,
    },
    Group{
        gid: i64,
        stream: String,
        topic: String,
    },
}

struct Task {
    id: i64,
    name: String,
    // 描述
    desc: String,
    // 到期时间
    deadline: i64,
    delegate: Delegate,
    // 回执列表, pg查询最后一个元素: select receipts[array_upper(receipts, 1)]
    receipts: Vec<TaskReceipt>
}
```

验收时会同步将结果和批示发送至Topic中.

```rust
struct TaskReceipt {
    tid: i64,
    status: String,
    desc: String,
    timestamp: i128,
}
```





## Database Schema

作为实验项目, 希望在各个方面都进行探索, 下面是Schema设计.

```pgsql
CREATE SCHEMA IF NOT EXISTS slep;
-- AWS RDS support this: https://docs.aws.amazon.com/AmazonRDS/latest/PostgreSQLReleaseNotes/postgresql-extensions.html
CREATE EXTENSION IF NOT EXISTS btree_gist;

CREATE TYPE slep.message_type AS ENUM ('unknown', 'md', 'img', 'video', 'audio');
CREATE TYPE slep.message_addr_type AS ENUM ('unknown', 'private', 'stream');

CREATE TABLE slep.message (
  id BIGINT NOT NULL,
  typ slep.message_type NOT NULL,
  addr_typ slep.message_addr_type NOT NULL,
  -- private: uid, stream: hash("[gid]stream")
  addr BIGINT NOT NULL,
  topic TEXT NOT NULL,
  content TEXT NOT NULL,
  sender BIGINT NOT NULL,
  receiver BIGINT,
  timestamp BIGINT NOT NULL,
  CONSTRAINT slep_message_pkey PRIMARY KEY (id)
  CONSTRAINT slep_message_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_message_id_idx ON slep.message (id);

CREATE TABLE slep.stream_settings (
  stream TEXT NOT NULL,
  gid BIGINT NOT NULL,
  des TEXT,
  rlevel SMALLINT NOT NULL DEFAULT 0 CHECK(rlevel >= 0),
  wlevel SMALLINT NOT NULL DEFAULT 0 CHECK(wlevel >= 0),
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
  CONSTRAINT slep_topic_settings_pkey PRIMARY KEY (hashkey)
  CONSTRAINT slep_topic_settings_conflict EXCLUDE USING gist (hashkey WITH =)
);
CREATE INDEX slep_topic_settings_idx ON slep.topic_settings (hashkey);

CREATE TABLE slep.message_read_status (
  uid BIGINT NOT NULL,
  -- TOPIC.hashkey
  addr_hash BIGINT NOT NULL,
  latest_mid BIGINT NOT NULL,
  CONSTRAINT slep_message_read_status_pkey PRIMARY KEY (uid, addr_hash),
  CONSTRAINT slep_message_read_status_conflict EXCLUDE USING gist (uid WITH =, addr_hash WITH =)
);
CREATE INDEX slep_message_read_status_uid_idx ON slep.message_read_status (uid);

CREATE TABLE slep.user_group (
  id BIGINT NOT NULL,
  pid BIGINT,
  name TEXT NOT NULL,
  des TEXT,
  CONSTRAINT slep_user_group_pkey PRIMARY KEY (id),
  CONSTRAINT slep_user_group_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_user_group_id_idx ON slep.user_group (id);

CREATE TABLE slep.group_member (
  gid BIGINT NOT NULL,
  uid BIGINT NOT NULL,
  level SMALLINT NOT NULL DEFAULT 0 CHECK(level > 0),

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

CREATE TYPE slep.office_automation_task_type AS ENUM ('unknown', 'group', 'private');

CREATE TABLE slep.office_automation_task (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  des TEXT,
  typ slep.office_automation_task_type NOT NULL,
  consignor BIGINT NOT NULL,
  deadline BIGINT NOT NULL DEFAULT 0,
  receipts slep.office_automation_task_receipt[],
  CONSTRAINT slep_office_automation_task_pkey PRIMARY KEY (id),
  CONSTRAINT slep_office_automation_task_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_office_automation_task_idx ON slep.office_automation_task (id);

CREATE TABLE slep.user (
  id BIGINT NOT NULL,
  name TEXT NOT NULL,
  CONSTRAINT slep_user_pkey PRIMARY KEY (id)
  CONSTRAINT slep_user_conflict EXCLUDE USING gist (id WITH =)
);
CREATE INDEX slep_user_id_idx ON slep.user (id);
```

### Query

```pgsql
-- Set Message
INSERT INTO slep.message (id, typ, addr_typ, addr, topic, content, sender, receiver, timestamp) VALUES ($1, $2::slep.message_type, $3::slep.message_addr_type, $4, $5, $6, $7, $8, $9) ON CONFLICT ON CONSTRAINT slep_message_pkey DO UPDATE SET typ= EXCLUDED.typ, addr_typ= EXCLUDED.addr_typ, addr= EXCLUDED.addr, topic= EXCLUDED.topic, content= EXCLUDED.content, sender= EXCLUDED.sender, receiver= EXCLUDED.receiver, timestamp= EXCLUDED.timestamp;

-- Get Private Messages
SELECT id, typ, addr_typ, addr, topic, content, sender, receiver FROM slep.message WHERE addr_typ = 'private' AND (addr = $1 OR sender = $1);

-- Get Stream Messages
SELECT id, typ, addr_typ, addr, topic, content, sender, receiver FROM slep.message WHERE addr_typ = 'stream' AND addr =ANY($1);

-- Get Messages By Addr $ Topic
{get messages} AND topic = $1;

-- Set Message Read Status
INSERT INTO slep.message_read_status (uid, addr_hash, latest_mid) VALUES ($1,$2,$3) ON CONFLICT ON CONSTRAINT slep_message_read_status_pkey DO UPDATE SET latest_mid = EXCLUDED.latest_mid;

-- Get Message Read Status
SELECT uid, addr_hash, latest_mid FROM slep.message_read_status WHERE uid = $1;

-- Set StreamSettings
INSERT INTO slep.stream_settings (stream, gid, des, rlevel, wlevel) VALUES ($1, $2, $3, $4, $5) ON CONFLICT ON CONSTRAINT slep_stream_settings_pkey DO UPDATE SET des= EXCLUDED.des, rlevel= EXCLUDED.rlevel, wlevel= EXCLUDED.wlevel;

-- Get StreamSettings
SELECT stream, gid, des, rlevel, wlevel FROM slep.stream_settings WHERE gid = $1;

-- Set TopicSettings
INSERT INTO slep.topic_settings (hashkey, associate_task_id, rlevel, wlevel) VALUES ($1, $2, $3, $4) ON CONFLICT ON CONSTRAINT slep_topic_settings_pkey DO UPDATE SET associate_task_id= EXCLUDED.associate_task_id, rlevel= EXCLUDED.rlevel, wlevel= EXCLUDED.wlevel;

-- Get TopicSettings
SELECT hashkey, associate_task_id, rlevel, wlevel FROM slep.topic_settings WHERE hashkey = $1;

-- Get Task List By consignor
SELECT id, name, des, typ, consignor, deadline, receipts[array_upper(receipts, 1)] FROM slep.office_automation_task WHERE consignor = $1;

-- Get Task List By Status
SELECT id, name, des, typ, consignor, deadline, receipts[array_upper(receipts, 1)] FROM slep.office_automation_task WHERE receipts[array_upper(receipts, 1)].status = $1;

-- Set Task
INSERT INTO slep.office_automation_task(id, name, des, typ, consignor, deadline) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT ON CONSTRAINT slep_office_automation_task_pkey DO UPDATE SET name= EXCLUDED.name, des= EXCLUDED.des, typ= EXCLUDED.typ, consignor= EXCLUDED.consignor, deadline= EXCLUDED.deadline;

-- Get Task Info
SELECT id, name, des, typ, consignor, deadline, receipts[array_upper(receipts, 1)] FROM slep.office_automation_task WHERE id = $1;

-- Add Task Receipt
UPDATE slep.office_automation_task SET receipts = receipts || ($1, $2, $3, $4)::slep.office_automation_task_receipt WHERE id = $5;

-- Get The Group To Which The User Belongs
SELECT gid, uid, level FROM slep.group_member WHERE uid =$1;

-- Get GroupMember List
SELECT gid, uid, level FROM slep.group_member WHERE gid =$1;

-- Get Sub Group
SELECT id, pid, name, des FROM slep.user_group WHERE pid = $1;

-- Set Group
INSERT INTO slep.user_group (id, pid, name, des) VALUES ($1, $2, $3, $4) ON CONFLICT ON CONSTRAINT slep_user_group_pkey DO UPDATE SET pid= EXCLUDED.pid, name= EXCLUDED.name, des= EXCLUDED.des;

-- Delete Group
Delete FROM slep.user_group WHERE id =$1;

-- Set GroupMember
INSERT INTO slep.group_member (gid, uid, level) VALUES ($1, $2, $3) ON CONFLICT ON CONSTRAINT slep_group_member_pkey DO UPDATE SET level= EXCLUDED.level;

-- Set UserInfo
INSERT INTO slep.user (id, name) VALUES ($1, $2) ON CONFLICT ON CONSTRAINT slep_user_pkey DO UPDATE SET name= EXCLUDED.name;
```
