# Slep Core Design

Author: wenjing

Data: 2022-11-28

Version: 0.0.1





## 内存数据结构

#### 组

```rust
struct Groups(HashMap<String,Group>)

struct Group{
    // 组信息
    group_info: GroupInfo,
    // 组成员列表    key: gid(组id)   value: GroupMember(组成员)
    group_members: HashMap<String, GroupMember>
    // streams列表  key: stream_name   value: StreamSettings(stream设置)
    streams: HashMap<String, Stream>
}

// 组信息
struct GroupInfo{
    // 组id
    id: i64,
    // 父组id
  	pid: i64,
    // 组名
    name: String,
    // 说明
    desc: String,
}

// 组成员
struct GroupMember {
    // 用户id
    uid: i64,
    // 用户等级
    level: u8,
}

```

#### Stream

```rust

struct Stream{
    // stream 设置
    stream_settings: StreamSettings,
}

// Stream设置
struct StreamSettings {
    // 描述
    desc: String,
    read_level: u8,
    write_level: u8
}
```

#### Topic

```rust
// topic 设置
struct Topics(HashMap<u64, TopicSettings>)

// topic设置
struct TopicSettings{
    // 关联OA任务
    associate_tasks: HashMap<i64, Task>,
    read_level: u8,
    write_level: u8
}
```



#### OA

```rust
enum Delegate{
    Private{
        // 委派者
        delegate: u8
    },
    Group{
        // 委派者
        delegate: u8
    },
    Stream{
        // 委派者
        delegate: u8,
        stream: Option<String>
    },
}

// 任务
struct Task{
    id: i64,
    name: String,
    // 描述
    desc: String,
    // 到期时间
    deadline: i64,
    // 委派类型: 0: Privtae, 1: Group, 2: Stream -> Topic
    delegate: Delegate,
    // 回执列表, pg查询最后一个元素: arr[array_upper(arr, 1)]
    receipts: Vec<TaskReceipt>
}

// 任务回执
struct TaskReceipt{
    tid: i64,
    // 状态: 0: 创建, 1: 待确认, 2: 通过, 3: 拒绝
    status: u8,
    desc: String,
    timestamp: i128
}
```



## Command

```rust
// 消息地址
enum Addr{
        Private(receiver, topic),
        // receiver: @某人
        Public(org_id, stream, topic, Option<receiver>)   
}

enum Command {
    /// ACK: trace_id
    ACK(i64),
    
    /// 用户重命名
    RenameUsername(trace_id, name),

    /// 发送消息
    SendMessage(
        trace_id,
        message_id,
      	Addr,
        content,
        timestamp,
    ),
    
   	/// 撤销消息
    RevokeMessage(
        trace_id,
        message_id,
      	Addr,
    ),
    
    /// 删除消息
    DeleteMessage(
        trace_id,
        message_id,
      	Addr,
    ),
    
    /// 添加任务回执
    AddTaskReceipt(trace_id, stream, topic, status),
    
    /// 任命审核员
    AppointReviewer(trace_id, stream, reviewer),
    /// 解雇审核员
    DismissReviewer(trace_id, stream, reviewer),

    /// 创建组
    CreateGroup(trace_id, Option<pid>, group_name, description),
    /// 解散组
    DismissGroup(trace_id, group_id),
    
    /// 更新组信息
    UpdateGroupInfo(trace_id, gid, group_name, description),

    /// 添加组员
    AddGroupMember(trace_id, group_id, uid),
    /// 移除组员
    DismissMember(trace_id, group_id, uid),

    /// Err: trace_id, error
    Err(i64, String),
}
```



## Task

```rust
// Action： 动作，指对这个枚举的操作，例如create、delete
enum Action{
    // 创建
    Create,
    // 添加
    Add,
    // 更新
    Update,
    // 删除
    Delete,
    // 任命
    Appointer,
    // 解雇、解散
    Dismiss,
    // 重命名
    Rename,
    // 发送
    Send,
    // 撤回
    Revoke,
}


enum Task{
    /// ACK: trace_id
    ACK(i64),
    
    /// 用户名变动
    /// action: Rename
    UserInfo(trace_id, ok, action, name),
    
    /// 消息变动
    /// action: Send, Revoke, Delete
    Message(trace_id, ok, action, org_id, stream, topic, message_id, Option<MessageData>)
    
    /// 任务回执变动
    /// action: Add
    TaskReceipt(trace_id, ok, action, reviewer, org_id, stream, topic, status),
    
    /// 审核人变动
    /// action: Appointer, Dismiss
    Reviewer(trace_id, ok, action, org_id, stream, reviewer),

    /// 组变动
    /// action: Create, Dismiss
    Group(trace_id, ok, Action, org_id, group_id),
    
    /// 组信息变动
    /// action: Update
    GroupInfo(trace_id, ok, Action, org_id, group_id, group_name, desc),
    
    /// 成员变动: trace_id, ok
    /// action: Add，Dismiss
    Member(trace_id, ok, Action, org_id, group_id, uid),
 
    /// Err: trace_id, error
    Err(i64, String) 
}
```






