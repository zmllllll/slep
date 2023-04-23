use super::TopicAction;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct TopicSettings {
    hashkey: i64,
    associate_task_id: i64,
    rlevel: i16,
    wlevel: i16,
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Hashkey {
    pub(crate) gid: Option<i64>,
    pub(crate) addr: String,
    pub(crate) topic: String,
}

impl From<TopicSettings> for (i64, Vec<TopicAction>) {
    fn from(value: TopicSettings) -> Self {
        let actions = vec![
            TopicAction::UpdateAssociateTask(value.associate_task_id),
            TopicAction::UpdateReadLevel(value.rlevel),
            TopicAction::UpdateReadLevel(value.wlevel),
        ];

        (value.hashkey, actions)
    }
}

impl From<Hashkey> for (i64, Vec<TopicAction>) {
    fn from(value: Hashkey) -> Self {
        let hashkey = payload::resources::gen_topic_key(value.gid, &value.addr, &value.topic);
        let actions = vec![
            TopicAction::UpdateGid(value.gid),
            TopicAction::UpdateAddr(value.addr),
            TopicAction::UpdateTopic(value.topic),
        ];
        (hashkey, actions)
    }
}
