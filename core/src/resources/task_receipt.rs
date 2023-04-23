use super::*;

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, TaskReceipt>> {
    type Ext = Extension<'a>;
    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        use collect::{Collect as _, CollectMap as _, GenCollector as _};
        match &self.action {
            GeneralAction::Update { id: _, resource } => {
                if let Some(topics) = ext.0.topics.get(&resource.hashkey) {
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
                    Err(anyhow::anyhow!("[generate TaskId] hashkey is none"))
                }
            }
            GeneralAction::Drop(_) => todo!(),
            _ => Ok(Receiver::None),
        }
    }
}
