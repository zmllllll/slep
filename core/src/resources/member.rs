use super::*;
use crate::storage::action::member::GroupMemberAction;

impl<'a> Generator<'a, Option<Updater>> for Command<GeneralAction<Postgres, GroupMember>> {
    fn generate(&self, _ext: i64) -> Option<Updater> {
        match &self.action {
            GeneralAction::Insert { id, resource } | GeneralAction::Upsert { id, resource } => {
                if let Some((uid, gid)) = id {
                    Some(Updater::Groups(
                        *gid,
                        vec![GroupAction::GroupMember(GroupMemberAction::UpdateLevel(
                            *uid,
                            resource.level,
                        ))],
                    ))
                } else {
                    todo!();
                }
            }
            GeneralAction::Update { id, resource } => Some(Updater::Groups(
                id.1,
                vec![GroupAction::GroupMember(GroupMemberAction::UpdateLevel(
                    id.0,
                    resource.level,
                ))],
            )),
            GeneralAction::Drop(id) => Some(Updater::Groups(
                id.1,
                vec![GroupAction::GroupMember(GroupMemberAction::Leave(id.0))],
            )),
        }
    }
}

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, GroupMember>> {
    type Ext = Extension<'a>;
    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        use collect::{Collect as _, CollectMap as _, GenCollector as _};
        match self.action {
            GeneralAction::Insert { id, resource: _ }
            | GeneralAction::Upsert { id, resource: _ } => {
                if let Some((_, gid)) = id {
                    Ok(Receiver::List(
                        (&ext.0.groups, gid)
                            .gen_collector()
                            .collect_map(|c| c.collect_all()),
                    ))
                } else {
                    Err(anyhow::anyhow!("[generate Member] id is none"))
                }
            }
            GeneralAction::Drop((_, gid)) => Ok(Receiver::List(
                (&ext.0.groups, gid.to_owned())
                    .gen_collector()
                    .collect_map(|c| c.collect_all()),
            )),
            _ => Ok(Receiver::None),
        }
    }
}
