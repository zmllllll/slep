use super::*;
use crate::check;

impl<'a> Generator<'a, Option<Updater>> for Command<GeneralAction<Postgres, Message>> {
    fn generate(&self, _ext: i64) -> Option<Updater> {
        None
    }
}

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, Message>> {
    type Ext = Extension<'a>;
    fn generate(&self, ext: Self::Ext) -> Result<Receiver> {
        match &self.action {
            GeneralAction::Insert { id: _, resource }
            | GeneralAction::Upsert { id: _, resource } => {
                Ok(Receiver::List(if resource.gid.is_some() {
                    ext.0
                        .groups
                        .iter()
                        .flat_map(|(_, group)| {
                            // tracing::info!("group: {group:?}");
                            if let Some(s) = group.get_stream().get(&resource.addr) {
                                group
                                    .get_members()
                                    .iter()
                                    .filter_map(|member| {
                                        // tracing::info!("member: {member:?}");
                                        // tracing::info!("stream: {s:?}");
                                        use check::Check as _;
                                        member.check_level(check::Constraint::Range(
                                            check::Compare::Le(check::UpperLimit(s.read_level)),
                                        ))
                                    })
                                    .collect()
                            } else {
                                HashSet::new()
                            }
                        })
                        .collect()
                } else {
                    HashSet::from([resource.sender, resource.addr.parse::<i64>()?])
                }))
            }
            // TODO: revoke
            GeneralAction::Drop(_id) => Ok(Receiver::None),
            _ => Ok(Receiver::None),
        }
    }
}
