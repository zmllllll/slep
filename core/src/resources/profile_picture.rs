use super::*;

impl<'a> Generator<'a, Result<Receiver>>
    for resource::Command<
        resource::GeneralAction<
            sqlx::Postgres,
            payload::resources::profile_picture::ProfilePicture,
        >,
    >
{
    type Ext = Extension<'a>;
    fn generate(&self, _ext: Self::Ext) -> Result<Receiver> {
        match self.action {
            resource::GeneralAction::Upsert { id, resource: _ } => {
                if let Some(id) = id {
                    Ok(Receiver::Single(id))
                } else {
                    Err(anyhow::anyhow!("[generate ProfilePicture] id is none"))
                }
            }
            _ => Ok(Receiver::None),
        }
    }
}

impl<'a> Generator<'a, Option<Updater>>
    for resource::Command<
        resource::GeneralAction<
            sqlx::Postgres,
            payload::resources::profile_picture::ProfilePicture,
        >,
    >
{
    fn generate(&self, _ext: i64) -> Option<Updater> {
        None
    }
}
