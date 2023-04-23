use super::*;
use crate::{
    constant,
    storage::action::{group::GroupInfoAction, member::GroupMemberAction},
};

impl<'a> Generator<'a, Option<Updater>> for Command<GeneralAction<Postgres, Group>> {
    fn generate(&self, ext: i64) -> Option<Updater> {
        match &self.action {
            GeneralAction::Insert { id, ref resource }
            | GeneralAction::Upsert { id, ref resource } => id.as_ref().map(|gid| {
                Updater::Groups(
                    *gid,
                    vec![
                        GroupAction::GroupInfo(GroupInfoAction::UpdateName(
                            resource.name.to_owned(),
                        )),
                        GroupAction::GroupInfo(GroupInfoAction::UpdatePid(resource.pid)),
                        GroupAction::GroupMember(GroupMemberAction::UpdateLevel(
                            ext,
                            constant::CREATOR,
                        )),
                    ],
                )
            }),
            GeneralAction::Update { id, resource } => Some(Updater::Groups(
                *id,
                vec![
                    GroupAction::GroupInfo(GroupInfoAction::UpdateName(resource.name.to_owned())),
                    GroupAction::GroupInfo(GroupInfoAction::UpdatePid(resource.pid)),
                ],
            )),
            GeneralAction::Drop(id) => Some(Updater::Groups(*id, vec![GroupAction::Leave(*id)])),
        }
    }
}

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, Group>> {
    type Ext = Extension<'a>;
    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        use collect::{Collect as _, CollectMap as _, GenCollector as _};
        match self.action {
            GeneralAction::Insert { id, resource: _ }
            | GeneralAction::Upsert { id, resource: _ } => {
                Ok(Receiver::List(if let Some(id) = id {
                    (&ext.0.groups, id)
                        .gen_collector()
                        .collect_map(|c| c.collect_all())
                } else {
                    return Err(anyhow::anyhow!("[filter Group] id is none"));
                }))
            }
            GeneralAction::Update { id, resource: _ } => Ok(Receiver::List(
                (&ext.0.groups, id)
                    .gen_collector()
                    .collect_map(|c| c.collect_all()),
            )),
            GeneralAction::Drop(id) => Ok(Receiver::List(
                (&ext.0.groups, id)
                    .gen_collector()
                    .collect_map(|c| c.collect_all()),
            )),
        }
    }
}
