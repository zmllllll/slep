use super::*;

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, Reviewer>> {
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
                    Err(anyhow::anyhow!("[generate Reviewer] id is none"))
                }
            }
            _ => Ok(Receiver::None),
        }
    }
}

impl<'a> Generator<'a, Option<Updater>> for Command<GeneralAction<Postgres, Reviewer>> {
    fn generate(&self, _uid: i64) -> Option<Updater> {
        None
    }
}
