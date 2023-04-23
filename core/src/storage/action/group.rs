use super::GroupAction;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Group {
    id: i64,
    pid: Option<i64>,
    name: String,
}

impl From<Group> for (i64, Vec<GroupAction>) {
    fn from(value: Group) -> Self {
        let actions = vec![
            GroupAction::GroupInfo(GroupInfoAction::UpdateName(value.name)),
            GroupAction::GroupInfo(GroupInfoAction::UpdatePid(value.pid)),
        ];

        (value.id, actions)
    }
}

/// GroupInfoAction: Pid, Name
#[derive(Debug)]
pub(crate) enum GroupInfoAction {
    UpdatePid(Option<i64>),
    UpdateName(String),
}

impl GroupInfoAction {
    pub(super) fn act(&mut self, gid: i64, groups: &mut crate::storage::group::Groups) {
        match self {
            GroupInfoAction::UpdatePid(pid) => {
                groups.entry(gid).or_default().group_info.update_pid(*pid)
            }
            GroupInfoAction::UpdateName(name) => groups
                .entry(gid)
                .or_default()
                .group_info
                .update_name(name.to_string()),
        }
    }
}
