//! Storage is used to save data to the core, it is only includes those data which can
//! make a decision that where to send the response
//!
//! `group`, `topic`, `stream` are specified data structure
//!
//! `updater` is used to modify them
//!
//! `action` shows all possible changes of them

use super::*;

pub(crate) mod action;
pub(crate) mod group;
pub(crate) mod stream;
pub(crate) mod topic;
pub(crate) mod updater;
pub(self) type Level = i16;

pub(crate) fn any_in_vec<T: std::fmt::Display>(set: Vec<T>) -> String {
    let mut i = 1;
    let mut any = String::new();
    for s in set.iter() {
        any = format!("{any} {s}");
        if i < set.len() {
            any = format!("{any},");
        }
        i += 1;
    }
    any
}

#[derive(Default)]
pub(crate) struct StorageManager {
    pub(crate) groups: group::Groups,
    pub(crate) topics: topic::Topics,
}

impl StorageManager {
    // pub(crate) async fn init(&mut self, pool: &sqlx::Pool<sqlx::Any>) {
    pub(crate) async fn init(mut self, pool: &sqlx::Pool<sqlx::Any>) -> Self {
        self.group_settings(pool).await;
        self.member_settings(pool).await;
        self.stream_settings(pool).await;
        let hashkeys = self.topic_settings(pool).await;
        self.hashkey_list(hashkeys, pool).await;

        tracing::info!("init groups: {:#?}", self.groups);
        tracing::info!("init topics: {:#?}", self.topics);
        self
    }

    pub(crate) async fn group_settings(&mut self, pg: &sqlx::Pool<sqlx::Any>) {
        let sql = "SELECT id, pid, name, des FROM slep.user_group;";
        let gs: Vec<action::group::Group> = sqlx::query_as(sql).fetch_all(pg).await.unwrap();
        tracing::info!("group into: {gs:#?}");
        gs.into_iter().for_each(|g| {
            let (gid, action): (i64, Vec<action::GroupAction>) = g.into();
            action
                .into_iter()
                .for_each(|mut a| a.update(gid, &mut self.groups));
        });
    }

    pub(crate) async fn member_settings(&mut self, pg: &sqlx::Pool<sqlx::Any>) {
        let sql = "SELECT gid, uid, level FROM slep.group_member;";

        let members: Vec<action::member::GroupMember> =
            sqlx::query_as(sql).fetch_all(pg).await.unwrap();
        members.into_iter().for_each(|gm| {
            let (gid, action): (i64, Vec<action::GroupAction>) = gm.into();
            action
                .into_iter()
                .for_each(|mut a| a.update(gid, &mut self.groups));
        });
    }

    pub(crate) async fn stream_settings(&mut self, pg: &sqlx::Pool<sqlx::Any>) {
        let sql = "SELECT stream, gid, rlevel, wlevel FROM slep.stream_settings;";

        let streams: Vec<action::stream_settings::StreamSettings> =
            sqlx::query_as(sql).fetch_all(pg).await.unwrap();
        streams.into_iter().for_each(|stream| {
            let (gid, action): (i64, Vec<action::GroupAction>) = stream.into();
            action
                .into_iter()
                .for_each(|mut a| a.update(gid, &mut self.groups));
        });
    }

    pub(crate) async fn topic_settings(&mut self, pg: &sqlx::Pool<sqlx::Any>) -> Vec<i64> {
        let sql = "SELECT hashkey, associate_task_id, rlevel, wlevel FROM slep.topic_settings
        WHERE associate_task_id is not NULL;";
        let ts: Vec<action::topic_settings::TopicSettings> =
            sqlx::query_as(sql).fetch_all(pg).await.unwrap();
        tracing::info!("ts: {ts:#?}");
        let mut hashkeys = Vec::new();
        ts.into_iter().for_each(|t| {
            let (hashkey, action): (i64, Vec<action::TopicAction>) = t.into();
            hashkeys.push(hashkey);
            action
                .into_iter()
                .for_each(|mut a| a.update(hashkey, &mut self.topics));
        });
        hashkeys
    }

    pub(crate) async fn hashkey_list(&mut self, hashkeys: Vec<i64>, pg: &sqlx::Pool<sqlx::Any>) {
        let gids = self
            .groups
            .iter()
            .map(|(gid, _)| gid.to_owned())
            .collect::<Vec<i64>>();
        let in_sql = any_in_vec(gids);
        tracing::info!("in_sql: {in_sql}");
        let sql = if in_sql.is_empty() {
            "SELECT gid, addr, topic FROM slep.message WHERE gid is NULL;".to_string()
        } else {
            format!(
                "SELECT gid, addr, topic FROM slep.message WHERE gid in ({in_sql}) OR gid is NULL;"
            )
        };
        tracing::info!("sql: {sql:?}");

        let hks: Vec<action::topic_settings::Hashkey> =
            sqlx::query_as(&sql).fetch_all(pg).await.unwrap();

        hks.into_iter().for_each(|t| {
            let hk = payload::resources::gen_topic_key(t.gid, &t.addr, &t.topic);

            if hashkeys.contains(&hk) {
                let (hashkey, action): (i64, Vec<action::TopicAction>) = t.into();

                action
                    .into_iter()
                    .for_each(|mut a| a.update(hashkey, &mut self.topics));
            }
        });
    }
}
