use super::*;

use std::collections::{HashMap, HashSet};

use crate::{error, sqlite_operator};

use super::{anyhow, connect_pg, Result, PG_POOL, SQLITE_POOL};
use resource::Resource;
use sqlx::Executor;

#[derive(Debug, sqlx::FromRow, Default)]
struct Timestamp {
    timestamp: i64,
}

pub(crate) async fn user(members: &str) -> Result<()> {
    if members.is_empty() {
        return Ok(());
    };
    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = format!("SELECT id, name, profile_picture, timestamp FROM slep.user WHERE id in ({members});");

        #[derive(Debug, sqlx::FromRow)]
        struct UserTemp {
            id: i64,
            name: String,
            profile_picture: Option<String>,
            timestamp: i64,
        }

        let temps: Vec<UserTemp> = sqlx::query_as(&sql)
            .fetch_all(pg)
            .await?;
        use resources::Resources as _;
        use payload::resources::user::User;

        let mut sqlite_tx = sqlite.begin().await?;

        for temp in temps {
            let user = User{
                name: temp.name,
                profile_picture: temp.profile_picture,
                timestamp: temp.timestamp,
            };
            <User as resource::Resource<sqlx::Sqlite>>::upsert(&user, &Some(temp.id), &mut sqlite_tx).await?;
        }
        sqlite_tx.commit().await?;
    };
    Ok(())
}

pub(crate) async fn message(uid: i64, levels: Levels) -> Result<String> {
    let mut topic_key = String::new();
    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT id FROM message ORDER BY id DESC limit 0,1;";
        #[derive(Debug, sqlx::FromRow, Default)]
        struct MessageId {
            id: i64,
        }
        let message_id: MessageId = sqlx::query_as(sql).fetch_one(sqlite).await.unwrap_or_default();
        #[derive(Debug, sqlx::FromRow)]
        struct Stream {
            stream: String,
            gid: i64,
            rlevel: i16,
            wlevel: i16,
        }

        impl Stream {
            fn gen_addr(&self) -> i64 {
                gen_addr(self.gid, &self.stream)
            }
        }

        let sql = "SELECT stream, gid, rlevel, wlevel FROM stream_settings;";
        let streams: Vec<Stream> = sqlx::query_as(sql).fetch_all(sqlite).await?;

        tracing::info!("streams: {streams:#?}");
        tracing::info!("levels: {levels:#?}");
        let mut ors = streams
        .into_iter()
        .filter(|stream| levels.contains_key(&stream.gid))
        .filter(|stream| levels.get(&stream.gid).unwrap() <= &stream.rlevel)
        .map(|stream| format!("(gid = {} AND addr = '{}')", stream.gid, stream.stream))
        .collect::<HashSet<String>>();
        let mut or_sql = any_in_hashset(ors, " OR ");
        if !or_sql.is_empty(){
            or_sql = format!("OR {or_sql}");
        }


        let sql = format!("SELECT id, gid, typ, addr, topic, content, sender, timestamp
            FROM slep.message WHERE id > $1
            AND (
                    (gid = NULL AND 
                        (addr = $2 OR sender = $3)
                    )
            {or_sql}
            );");
            error!("pull message sql: {:#?}", sql);
        let messages: Vec<message::Message> = sqlx::query_as(&sql)
            .bind(message_id.id)
            .bind(uid.to_string())
            .bind(uid)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for message in messages{
            use resources::Resources as _;
            use payload::resources::message::Message;

            let m = Message{
                gid: message.gid,
                typ: message.typ.fmt(),
                addr: message.addr,
                topic: message.topic,
                content: message.content,
                sender: message.sender,
                timestamp:  message.timestamp
            };
            <Message as resource::Resource<sqlx::Sqlite>>::upsert(&m, &Some(message.id), &mut sqlite_tx).await?;
        }

        sqlite_tx.commit().await?;

        #[derive(Debug, sqlx::FromRow)]
        struct Topic {
            gid: i64,
            addr: String,
            topic: String,
        }
        let sql = "SELECT gid, addr, topic FROM message;";

        let topics:Vec<Topic> = sqlx::query_as(sql).fetch_all(sqlite).await.unwrap_or_default();
        let mut topic_set = HashSet::new();
        topics.into_iter().for_each(|t|{
                topic_set.insert(gen_topic_key(Some(t.gid), &t.addr, &t.topic)) ;
        });
        topic_key = pull::any_in_hashset(topic_set,",");

    }else{
        return Err(anyhow!(error::Error::PoolGetFailed("pg or sqlite".to_owned())))
    };
    Ok(topic_key)
}

type Levels = HashMap<i64, i16>;

pub(crate) async fn group_member(uid: i64) -> Result<(String, String, Levels)> {
    let mut groups = String::new();
    let mut members = String::new();
    let mut levels = HashMap::new();

    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT timestamp FROM group_member WHERE uid =$1 ORDER BY timestamp DESC limit 0,1;";
        let timestamp: Timestamp = sqlx::query_as(sql).bind(uid).fetch_one(sqlite).await.unwrap_or_default();

        let sql = "SELECT gid, uid, level, timestamp FROM slep.group_member 
        WHERE gid in (SELECT gid FROM slep.group_member WHERE uid = $1 AND timestamp >= $2);";
        #[derive(Debug, sqlx::FromRow)]
        struct GroupMember {
            gid: i64,
            uid: i64,
            level: i16,
            timestamp: i64,
        }

        let group_members: Vec<GroupMember> = sqlx::query_as(sql)
            .bind(uid)
            .bind(timestamp.timestamp)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for member in group_members{
            use resources::Resources as _;
            use payload::resources::member::GroupMember;

            let m = GroupMember{
                level: member.level,
                timestamp: member.timestamp,
            };
            <GroupMember as resource::Resource<sqlx::Sqlite>>::upsert(&m, &Some((member.uid,member.gid)), &mut sqlite_tx).await?;
        }

        sqlite_tx.commit().await?;

        let sql = "SELECT gid, uid, level FROM group_member;";
        #[derive(Debug, sqlx::FromRow)]
        struct GidUid {
            gid: i64,
            uid: i64,
            level: i16
        }
        let gids_uids: Vec<GidUid> = sqlx::query_as(sql).fetch_all(sqlite).await.unwrap_or_default();
        let mut gids = HashSet::new();
        let mut uids = HashSet::new();
        gids_uids.into_iter().for_each(|gid_uid|{
            gids.insert(gid_uid.gid);
            uids.insert(gid_uid.uid);
            if gid_uid.uid == uid{
                levels.insert(gid_uid.gid,gid_uid. level);
            }
        });
        groups = pull::any_in_hashset(gids,",");
        members = pull::any_in_hashset(uids,",");
        if members.is_empty(){
            members = uid.to_string()
        }
    };

    Ok((groups, members, levels))
}

pub(crate) fn any_in_hashset<T: std::fmt::Display>(set: HashSet<T>, placeholder: &str) -> String {
    let mut i = 1;
    let mut any = String::new();
    for s in set.iter() {
        any = format!("{any} {s}");
        if i < set.len() {
            any = format!("{any}{placeholder}");
        }
        i += 1;
    }
    any
}

pub(crate) async fn group(groups: &str) -> Result<()> {
    if groups.is_empty() {
        return Ok(());
    }
    if let Some(pg) = PG_POOL.get() &&
     let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT timestamp FROM user_group ORDER BY timestamp DESC limit 0,1;";
        let timestamp: Timestamp = sqlx::query_as(sql).fetch_one(sqlite).await.unwrap_or_default();

        let sql = format!( "SELECT id, pid, name, des, timestamp FROM slep.user_group WHERE timestamp >= $1 AND id in ({groups});");
        #[derive(Debug, sqlx::FromRow)]
        struct Group {
            id: i64,
            pid: Option<i64>,
            name: String,
            des: Option<String>,
            timestamp: i64,
        }

        let groups: Vec<Group> = sqlx::query_as(&sql)
            .bind(timestamp.timestamp)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for group in groups {
            use resources::Resources as _;
            use payload::resources::group::Group;

            let g = Group{
                pid: group.pid,
                name: group.name,
                des: group.des,
                timestamp: group.timestamp,
            };
            <Group as resource::Resource<sqlx::Sqlite>>::upsert(&g, &Some(group.id), &mut sqlite_tx).await?;
        }

        sqlite_tx.commit().await?;
    };
    Ok(())
}

pub(crate) async fn stream_settings(groups: &str) -> Result<()> {
    if groups.is_empty() {
        return Ok(());
    }
    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT timestamp FROM stream_settings ORDER BY timestamp DESC limit 0,1;";
        let timestamp: Timestamp = sqlx::query_as(sql).fetch_one(sqlite).await.unwrap_or_default();

        let sql = format!( "SELECT stream, gid, des, rlevel, wlevel, timestamp FROM slep.stream_settings WHERE timestamp >= $1 AND gid in ({groups});");
        #[derive(Debug, sqlx::FromRow)]
        struct StreamSettings {
            stream: String,
            gid: i64,
            des: Option<String>,
            rlevel: i16,
            wlevel: i16,
            timestamp: i64,
        }

        let streams: Vec<StreamSettings> = sqlx::query_as(&sql)
            .bind(timestamp.timestamp)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for stream in streams{
            use resources::Resources as _;
            use payload::resources::stream::StreamSettings;

            let s = StreamSettings{
                des: stream.des,
                rlevel: stream.rlevel,
                wlevel: stream.wlevel,
                timestamp: stream.timestamp,
            };
            <StreamSettings as resource::Resource<sqlx::Sqlite>>::upsert(&s, &Some((stream.stream,stream.gid)), &mut sqlite_tx).await?;
        }

        sqlite_tx.commit().await?;
    };
    Ok(())
}

pub(crate) async fn topic_settings(topic_keys: &str) -> Result<()> {
    if topic_keys.is_empty() {
        return Ok(());
    };
    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT timestamp FROM topic_settings ORDER BY timestamp DESC limit 0,1;";
        let timestamp: Timestamp = sqlx::query_as(sql).fetch_one(sqlite).await.unwrap_or_default();

        let sql =format!("SELECT hashkey, associate_task_id, rlevel, wlevel, timestamp 
        FROM slep.topic_settings WHERE timestamp >= $1 AND hashkey in ({topic_keys});");
        #[derive(Debug, sqlx::FromRow)]
        struct TopicSettings {
            hashkey: i64,
            associate_task_id: Option<i64>,
            rlevel: i16,
            wlevel: i16,
            timestamp: i64,
        }

        let topics: Vec<TopicSettings> = sqlx::query_as(&sql)
            .bind(timestamp.timestamp)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for topic in topics{
            use resources::Resources as _;
            use payload::resources::topic::TopicSettings;

            let t = TopicSettings{
                associate_task_id: topic.associate_task_id,
                rlevel: topic.rlevel,
                wlevel: topic.wlevel,
                timestamp: topic.timestamp,
            };
            <TopicSettings as resource::Resource<sqlx::Sqlite>>::upsert(&t, &Some(topic.hashkey), &mut sqlite_tx).await?;
        }

        sqlite_tx.commit().await?;
    };
    Ok(())
}

pub(crate) async fn task(uid: i64) -> Result<()> {
    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT timestamp FROM office_automation_task ORDER BY timestamp DESC limit 0,1;";
        let timestamp: Timestamp = sqlx::query_as(sql).fetch_one(sqlite).await.unwrap_or_default();

        let sql =
            "SELECT id, name, des, typ, consignor, deadline, timestamp FROM slep.office_automation_task WHERE timestamp >= $1;";

        let tasks: Vec<task::Task> = sqlx::query_as(sql)
            .bind(timestamp.timestamp)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for task in tasks{
            use resources::Resources as _;
            use payload::resources::office_automation_task::OATask;

            let t = OATask{
                name: task.name,
                des: task.des,
                typ: task.typ.fmt(),
                consignor: task.consignor,
                deadline:task.deadline,
                timestamp: task.timestamp,
            };
            <OATask as resource::Resource<sqlx::Sqlite>>::upsert(&t, &Some(task.id), &mut sqlite_tx).await?;
        }

        sqlite_tx.commit().await?;
    };
    Ok(())
}

pub(crate) async fn task_receipt(uid: i64) -> Result<()> {
    if let Some(pg) = PG_POOL.get() &&
    let Some(sqlite) = unsafe { SQLITE_POOL.get() } {
        let sql = "SELECT timestamp FROM office_automation_task_receipt ORDER BY timestamp DESC limit 0,1;";
        let timestamp: Timestamp = sqlx::query_as(sql).fetch_one(sqlite).await.unwrap_or_default();

        let sql =
            "SELECT id, receipts FROM slep.office_automation_task WHERE timestamp >= $1;";

        let receipts: Vec<task::TaskReceipt> = sqlx::query_as(sql)
            .bind(timestamp.timestamp)
            .fetch_all(pg)
            .await?;

        let mut sqlite_tx = sqlite.begin().await?;

        for list in receipts{
            if let Some(receipts) = list.receipts{
                for receipt in receipts.iter(){
                    use resources::Resources as _;
            use payload::resources::task_receipt::SqliteTaskReceipt;

            let t = SqliteTaskReceipt{
                executor: receipt.executor,
                status: receipt.status.fmt(),
                des: Some(receipt.des.clone()),
                timestamp: receipt.timestamp,
            };
            <SqliteTaskReceipt as resource::Resource<sqlx::Sqlite>>::upsert(&t, &Some(list.id), &mut sqlite_tx).await?;
                }
            }
        }

        sqlite_tx.commit().await?;
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn pull() {
        let pg_pool = sqlx::Pool::<sqlx::Postgres>::connect(
            "postgresql://localhost?dbname=slep&user=postgres&password=123456",
        )
        .await
        .unwrap();
        let sql = "SELECT
        id,
        gid,
        typ,
        addr,
        topic,
        content,
        sender,
        timestamp
      FROM
        slep.message
      WHERE
            gid = 7029392459348324346
            AND addr = 'qwj_stream';";
        let messages: Vec<message::Message> =
            sqlx::query_as(sql).fetch_all(&pg_pool).await.unwrap();
        println!("res: {messages:#?}");
    }
}
