use super::*;

// hash("Gid+(Stream or uid)+Topic")
// hashkey: i64,
#[derive(Debug, Default)]
pub(crate) struct Topics(pub(crate) HashMap<i64, topic::TopicSettings>);

impl Deref for Topics {
    type Target = HashMap<i64, TopicSettings>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Topics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TopicSettings {
    // 关联OA任务
    pub(crate) gid: Option<i64>,
    // stream or uid
    pub(crate) addr: String,
    pub(crate) topic: String,
    pub(crate) associate_task: i64,
    read_level: Level,
    write_level: Level,
}

impl TopicSettings {
    pub(crate) fn update_gid(&mut self, gid: Option<i64>) {
        self.gid = gid
    }
    pub(crate) fn update_addr(&mut self, addr: String) {
        self.addr = addr
    }
    pub(crate) fn update_topic(&mut self, topic: String) {
        self.topic = topic
    }
    pub(crate) fn update_associate_task(&mut self, associate_task: i64) {
        self.associate_task = associate_task
    }

    pub(crate) fn update_read_level(&mut self, read_level: i16) {
        self.read_level = read_level
    }
    pub(crate) fn update_write_level(&mut self, write_level: i16) {
        self.write_level = write_level
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Status {
    Unknown,
    Created,
    Pending,
    Confirmed,
    Blocked,
    Failed,
}
