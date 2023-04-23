use super::*;

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, OATask>> {
    type Ext = Extension<'a>;
    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        use collect::{Collect as _, CollectMap as _, GenCollector as _};
        match &self.action {
            GeneralAction::Insert { id, resource } => {
                if let Some(id) = id {
                    if let Some((_, topics)) = &ext
                        .0
                        .topics
                        .iter()
                        .find(|(_, topics)| &topics.associate_task == id)
                    {
                        match resource.typ.as_str() {
                            "private" => Ok(Receiver::List(HashSet::from([
                                resource.consignor,
                                topics.addr.parse::<i64>()?,
                            ]))),
                            "group" => {
                                if let Some(gid) = topics.gid {
                                    Ok(Receiver::List(
                                        (&ext.0.groups, gid)
                                            .gen_collector()
                                            .collect_map(|c| c.collect_all()),
                                    ))
                                } else {
                                    Err(anyhow::anyhow!(
                                        "[generate Task] type is group but gid is none"
                                    ))
                                }
                            }
                            _ => Ok(Receiver::None),
                        }
                    } else {
                        Ok(Receiver::None)
                    }
                } else {
                    Err(anyhow::anyhow!("[generate Task] id is none"))
                }
            }
            _ => Ok(Receiver::None),
        }
    }
}
