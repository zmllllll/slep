use super::GroupAction;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct GroupMember {
    gid: i64,
    uid: i64,
    level: i16,
}

impl From<GroupMember> for (i64, Vec<GroupAction>) {
    fn from(value: GroupMember) -> Self {
        let actions = vec![GroupAction::GroupMember(GroupMemberAction::UpdateLevel(
            value.uid,
            value.level,
        ))];

        (value.gid, actions)
    }
}

/// GroupMemberAction: Level
#[derive(Debug)]
pub(crate) enum GroupMemberAction {
    UpdateLevel(i64, i16),
    Leave(i64),
}

impl GroupMemberAction {
    pub(super) fn update(&mut self, gid: i64, groups: &mut crate::storage::group::Groups) {
        if let GroupMemberAction::UpdateLevel(u, l) = self {
            groups
                .entry(gid)
                .or_default()
                .group_members
                .entry(*u)
                .or_default()
                .update(*l)
        }
    }

    pub(super) fn leave(&mut self, gid: i64, groups: &mut crate::storage::group::Groups) {
        if let GroupMemberAction::Leave(u) = self {
            if let Some(group) = groups.get_mut(&gid) {
                let _ = group.group_members.remove(u);
            }
        }
    }
}
