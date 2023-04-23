use super::*;

pub(crate) mod group;
pub(crate) mod member;
pub(crate) mod stream_settings;
pub(crate) mod topic_settings;

/// GroupAction: GroupInfo, GroupMember, Stream
#[derive(Debug)]
pub(crate) enum GroupAction {
    GroupInfo(group::GroupInfoAction),
    GroupMember(member::GroupMemberAction),
    Stream(stream_settings::StreamAction),
    Leave(i64),
}

impl GroupAction {
    pub(crate) fn update(&mut self, gid: i64, groups: &mut super::group::Groups) {
        match self {
            GroupAction::GroupInfo(i) => i.act(gid, groups),
            GroupAction::GroupMember(m) => m.update(gid, groups),
            GroupAction::Stream(s) => s.update(gid, groups),
            _ => {}
        }
    }

    pub(crate) fn leave(&mut self, gid: i64, groups: &mut super::group::Groups) {
        match self {
            GroupAction::Leave(m) => {
                let _ = groups.remove(m);
            }
            GroupAction::GroupMember(m) => m.leave(gid, groups),
            GroupAction::Stream(s) => s.leave(gid, groups),
            _ => {}
        }
    }
}

/// TopicAction: AssociateTask, ReadLevel, WriteLevel
#[derive(Debug)]
pub(crate) enum TopicAction {
    UpdateGid(Option<i64>),
    UpdateAddr(String),
    UpdateTopic(String),
    UpdateAssociateTask(i64),
    UpdateReadLevel(i16),
    #[allow(dead_code)]
    UpdateWriteLevel(i16),
    Leave,
}

impl TopicAction {
    pub(crate) fn update(&mut self, hash_key: i64, topics: &mut topic::Topics) {
        match self {
            TopicAction::UpdateGid(gid) => topics.entry(hash_key).or_default().update_gid(*gid),
            TopicAction::UpdateAddr(addr) => topics
                .entry(hash_key)
                .or_default()
                .update_addr(addr.to_string()),
            TopicAction::UpdateTopic(t) => topics
                .entry(hash_key)
                .or_default()
                .update_topic(t.to_string()),
            TopicAction::UpdateAssociateTask(at) => topics
                .entry(hash_key)
                .or_default()
                .update_associate_task(*at),
            TopicAction::UpdateReadLevel(rl) => {
                topics.entry(hash_key).or_default().update_read_level(*rl)
            }
            TopicAction::UpdateWriteLevel(wl) => {
                topics.entry(hash_key).or_default().update_write_level(*wl)
            }
            _ => {}
        }
    }

    pub(super) fn leave(&mut self, hash_key: i64, topics: &mut topic::Topics) {
        if let TopicAction::Leave = self {
            let _ = topics.remove(&hash_key);
        }
    }
}
