use super::*;

// hash("Gid+Stream+Topic")
// hashkey: i64,

#[derive(Debug, Clone, Default)]
pub(super) struct TopicSettings {
    // 关联OA任务
    associate_task: Option<i64>,
    read_level: i16,
    write_level: i16,
}
struct Task {
    task_id: i64,
    name: String,
    // 描述
    desc: String,
    // 到期时间
    deadline: i64,
    // 委派类型: 0: Private, 1: Group, 2: Stream -> Topic
    delegate: Delegate,
    // 委派人
    consignor: i64,
    // 回执列表, pg查询最后一个元素: arr[array_upper(arr, 1)]
    receipts: Vec<TaskReceipt>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) enum Delegate {
    Private,
    Group,
    Stream { stream: Option<String> },
}

#[derive(Deserialize, Debug)]
pub(super) struct TaskReceipt {
    task_id: u32,
    // 回执执行人
    executor: i64,
    // 状态: 0: 创建, 1: 待确认, 2: 通过, 3: 拒绝
    status: u8,
    desc: String,
    timestamp: i128,
}
