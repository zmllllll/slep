use super::*;

impl<'a> Generator<'a, Result<Receiver>> for Command<GeneralAction<Postgres, ReadStatus>> {
    type Ext = Extension<'a>;
    fn generate(&self, _ext: Self::Ext) -> Result<Receiver> {
        match &self.action {
            GeneralAction::Upsert { id: _, resource } => Ok(resource.addr.uid.into()),
            _ => Ok(Receiver::None),
        }
    }
}
