use super::*;

impl<'a> Generator<'a, Option<Updater>> for Command<GeneralAction<Postgres, TopicLevel>> {
    fn generate(&self, _ext: i64) -> Option<Updater> {
        match &self.action {
            GeneralAction::Insert { id, resource } | GeneralAction::Upsert { id, resource } => {
                id.as_ref().map(|hashkey| {
                    Updater::Topics(
                        *hashkey,
                        vec![
                            TopicAction::UpdateReadLevel(resource.rlevel),
                            TopicAction::UpdateWriteLevel(resource.wlevel),
                        ],
                    )
                })
            }
            GeneralAction::Update { id, resource } => Some(Updater::Topics(
                *id,
                vec![
                    TopicAction::UpdateReadLevel(resource.rlevel),
                    TopicAction::UpdateWriteLevel(resource.wlevel),
                ],
            )),
            GeneralAction::Drop(id) => Some(Updater::Topics(*id, vec![TopicAction::Leave])),
        }
    }
}

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, TopicLevel>> {
    type Ext = Extension<'a>;
    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        use collect::{Collect as _, CollectMap as _, GenCollector as _};
        match &self.action {
            GeneralAction::Insert { id, resource: _ }
            | GeneralAction::Upsert { id, resource: _ } => {
                if let Some(hashkey) = id {
                    if let Some(topics) = ext.0.topics.get(hashkey) {
                        if let Some(gid) = topics.gid {
                            Ok(Receiver::List(
                                (&ext.0.groups, gid)
                                    .gen_collector()
                                    .collect_map(|c| c.collect_all()),
                            ))
                        } else {
                            Ok(Receiver::List(HashSet::from([
                                ext.1,
                                topics.addr.parse::<i64>()?,
                            ])))
                        }
                    } else {
                        Err(anyhow::anyhow!("[generate TopicLevel] hashkey is none"))
                    }
                } else {
                    Err(anyhow::anyhow!("[generate TopicLevel] id is none"))
                }
            }
            _ => Ok(Receiver::None),
        }
    }
}
