use super::*;
use crate::storage::action::stream_settings::StreamAction;

impl<'a> Generator<'a, Option<Updater>> for Command<GeneralAction<Postgres, StreamLevel>> {
    fn generate(&self, _ext: Self::Ext) -> Option<Updater> {
        match &self.action {
            GeneralAction::Insert { id, resource } | GeneralAction::Upsert { id, resource } => {
                if let Some((stream, gid)) = id {
                    Some(Updater::Groups(
                        *gid,
                        vec![
                            GroupAction::Stream(StreamAction::UpdateReadLevel(
                                stream.to_string(),
                                resource.rlevel,
                            )),
                            GroupAction::Stream(StreamAction::UpdateWriteLevel(
                                stream.to_string(),
                                resource.wlevel,
                            )),
                        ],
                    ))
                } else {
                    todo!();
                }
            }
            GeneralAction::Update { id, resource } => Some(Updater::Groups(
                id.1,
                vec![
                    GroupAction::Stream(StreamAction::UpdateReadLevel(
                        id.0.to_string(),
                        resource.rlevel,
                    )),
                    GroupAction::Stream(StreamAction::UpdateWriteLevel(
                        id.0.to_string(),
                        resource.wlevel,
                    )),
                ],
            )),
            GeneralAction::Drop(id) => Some(Updater::Groups(
                id.1,
                vec![GroupAction::Stream(StreamAction::Leave(id.0.to_string()))],
            )),
        }
    }
}

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, StreamLevel>> {
    type Ext = Extension<'a>;

    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        use collect::{Collect as _, CollectMap as _, GenCollector as _};
        match &self.action {
            GeneralAction::Insert { id, resource: _ }
            | GeneralAction::Upsert { id, resource: _ } => {
                if let Some((_, gid)) = id {
                    Ok(Receiver::List(
                        (&ext.0.groups, gid.to_owned())
                            .gen_collector()
                            .collect_map(|c| c.collect_all()),
                    ))
                } else {
                    Err(anyhow::anyhow!("[generate StreamLevel] id is none"))
                }
            }
            _ => Ok(Receiver::None),
        }
    }
}
