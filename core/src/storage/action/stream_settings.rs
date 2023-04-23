use super::GroupAction;
use crate::storage::Level;

#[derive(Debug, sqlx::FromRow, Default)]
pub(crate) struct StreamSettings {
    stream: String,
    gid: i64,
    rlevel: i16,
    wlevel: i16,
}

impl From<StreamSettings> for (i64, Vec<GroupAction>) {
    fn from(value: StreamSettings) -> Self {
        let stream = value.stream.as_str();
        let actions = vec![
            GroupAction::Stream(StreamAction::UpdateReadLevel(
                stream.to_owned(),
                value.rlevel,
            )),
            GroupAction::Stream(StreamAction::UpdateReadLevel(
                stream.to_owned(),
                value.wlevel,
            )),
        ];

        (value.gid, actions)
    }
}

/// StreamAction: ReadLevel, WriteLevel
#[derive(Debug)]
pub(crate) enum StreamAction {
    UpdateReadLevel(String, Level),
    #[allow(dead_code)]
    UpdateWriteLevel(String, Level),
    Leave(String),
}

impl StreamAction {
    pub(super) fn update(&mut self, gid: i64, groups: &mut crate::storage::group::Groups) {
        match self {
            StreamAction::UpdateReadLevel(name, rl) => groups
                .entry(gid)
                .or_default()
                .streams
                .entry(name.to_string())
                .or_default()
                .update_read_level(*rl),
            StreamAction::UpdateWriteLevel(name, wl) => groups
                .entry(gid)
                .or_default()
                .streams
                .entry(name.to_string())
                .or_default()
                .update_write_level(*wl),
            _ => {}
        }
    }

    pub(super) fn leave(&mut self, gid: i64, groups: &mut crate::storage::group::Groups) {
        if let StreamAction::Leave(u) = self {
            if let Some(group) = groups.get_mut(&gid) {
                let _ = group.streams.remove(u);
            }
        }
    }
}
