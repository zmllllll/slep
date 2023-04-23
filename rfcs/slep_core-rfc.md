- Feature Name: (slep_core)
- Start Date: (2022-11-9)
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)

# Summary

[summary]: #summary

A communications server dedicated to handling dynamic tasks and distributing results.

# Motivation

[motivation]: #motivation

The instant messaging system needs a communication server to receive requests from the front-end, and then centrally process dynamic tasks  according to the type of requests, such as sending messages, modifying  stream or topic attributes, modifying members and reviewing status and  other non-immediate tasks. Under the condition of good network and  stable concurrency, the communication server implemented completely  according to the design plan can work in an orderly manner, and  basically there will be no logical errors.

# Guide-level explanation

[guide-level-explanation]: #guide-level-explanation

The overall process: 

- receive the task
- execute the task
- extract the user list based on the visibility of the stream and the type of the task,  and send the result to these clients through SSE.

# Reference-level explanation

[reference-level-explanation]: #reference-level-explanation

Whenever the communication establishes a connection, the connection is  stored in the map as the value, and the user name is used as the key.  The task sent by the client needs to carry the information of the user  list to tell the server to which clients the result should be sent back. Of course, if this information is removed, the core will extract the  list by itself, which may increase the binding between the server and  the business. It is recommended to implement this operation on the front-end.

The execution tasks are basically some SQL operations, and there are not many, and can be done directly on the core.

### 数据结构

```rust
struct User{
    uid: String,
    username: String,
   	
    //TBD
    org_id: String
}

struct Group{
    group_id: String,
    group_name: String,
    //TBD
    role: i8,
    description: String,
    //TBD
    org_id: String
}

struct GroupMember{
    group_id: String,
    uid: String,
    //TBD
    org_id,
}

struct StreamScope{
    stream_name: String,
    group_list: Vec<String>,
}

// stream
struct StreamManager{
    stream_name: String,
    group_list: Vec<String>
}

//TBD
struct TopicType{
    // hash(stream + topic)
    id: String,
    // 0: reject, 1: complete, 3: under review, 4:  developing, 5: discussion group
    typ: i8
}

struct TopicReview{
    // hash(stream + topic)
    id: String,
    reviewer: String,
    // 1: yes, 0: no
    vote: i8,
    //TBD
    org_id: String
}

struct Message{
    message_id: String,
    stream: String,
    topic: String,
    sender: String,
    receiver: String,
    
    //TBD
    org_id: String,
    // del: -1
    typ: i8,
    content: String,
    timestamp: String,
}
```

### 连接管理

首先要满足即时通讯，所以使用全双工通讯，这里的客户端与云端使用WebSocket协议。



#### 用户登录

**ws_login**	Get/ :token/:uuid

> 主要作用是校验用户token，uuid用来区分登录的设备
>
> 校验成功后，http升级为websocket协议，之后所有的交互都围绕websocket

##### web框架选择

- Rocket

  - 优点
    - 入门（上手简单），基于宏编程
    - 成熟度高、稳定性好
    - 类型安全、高速迭代
    - 文档齐全
  - 缺点
    - 它的易用性是通过宏来实现的，这种方式看似比较简单，本质上是把复杂度掩盖了，对于比较大的开发项目中，或者是想要控制每一块细节的、需要定制化的场景下是不友好的
    - 它是自成一体系的，和rust社区中其他的一些框架不太好结合

- Wrap

  - 优点
    - 新兴的轻量级Web框架
    - 基于hyper框架实现，和hyper作者是同一人
    - 通过组合满足不同的需求，适合写中间件
    - 性能好
    - 上手简单
    - 适合小而美的项目
  - 缺点
    - 文档缺失

- Actix-web

  - 优点

    - 在actix基础上开发的，隐藏了一些复杂度，提供了相对简单的接口

      > actix这个库提供了一种特殊的并发编程模型——actor model。

    - 成熟度高，健壮性

    - 速度最快，适合对性能有极端要求的场景。这归功于actor模型充分地利用了所有资源

  - 缺点

    - 不易使用，为了掩盖复杂度做了大量的宏

- Axum

  - 优点

    - tokio官方项目，基本算是rust下异步编程的标准

    - 简单，通过类型系统来实现的API易用性，调试起来容易

    - 基于tower实现，代码简洁，非常健壮。可以重用tower生态一整套的中间件

      > tower是一个用于构建健壮的网络客户端和服务器的底层框架

    - 速度快

  - 缺点

    - 是比较新的框架，文档缺失，但example比较多

**选择：**Axum

虽然从时间维度上讲Axum比较年轻，但是它的基础很牢固，它所依赖的tokio和tower都是很成熟的，且是这个方向的标准。高性能、易用、没有大量宏，符合人体工程学。

##### 异步编程框架选择

rust异步编程包括两部分，一部分是rust标准库的future，另一部分是运行future的主体，也就是谁来运行这个future，运行这个future需要一个runtime，目前tokio无疑是最优秀的runtime。



#### 隔离控制

为了可以实时、完整、隔离的控制用户在即时通讯系统内的所有活动，必须采用多线程并发，也就是依赖channel开进行线程间通信。

普通的线程并发模型，他们的线程间通信都是通过共享内存的方式进行，典型的方法是，创建一个全局共享的变量，通过加锁机制保证数据在并发环境下的线程安全，从而实现并发线程间的通信。而channel是通过通信的方式共享内存。

##### channel通信

由于采用了tokio runtime，并且不希望channel由于buffer的上限阻塞了command的发送，因此使用到tokio库中基于同步原语支持的异步环境下的多生产单消费通道，这个通道没有显式的buffer上限，因此不会阻塞住。

```rust
tokio::sync::mpsc::unbounded_channel
```

主进程下开启一个线程专门处理UserSessions

> LocalSet允许未实现Send的Future在相同的线程下运行

线程中保存着一个全局的Users列表，相当于一个用户连接的管理中枢

```rust
UserList{
    uid: UserConnections{
        device_id: Connection{
            heartbeat,
            websocket,
            task_queue,
        }
    }
}
```

UserSessionsSender和UserSessionsReceiver传入LocalSet中专门执行 **连接管理** 和 **隔离控制**，

LocalSet中的线程独立运行这部分事务：

- 接收UserSessionsSender发送过来的一系列指令，这些指令通过UserSessionsReceiver接收
- 根据指令中的uid和device_id，从UserSessions中取出对应的Connection，做后续的处理
- 传入的UserSessionsSender的作用只有一个，就是当Outcome发送超时时，用它发送一个TimeOut会话

总共需要用到三个通道

- UserSessionsChannel（User会话通道）

  通道类型：UserSessions

  使用范围：

  - sender：
    - 作为路由中间件安插在用户登录环节
      1. 初始化操作——负责发送Join会话
      2. 在监控websocket客户端的流数据请求时，根据模式匹配到的Message项，发送相应的UserSessions
      3. 在监控WebsocketSessionsChannel时，监听到Disconnect时，发送一个Disconnect会话
    - 在LocalSet线程中作为一个全局参数local_sender
      1. 当Outcome发送给websocket客户端后迟迟没有收到ACK时，引发超时（需要设置超时时间），local_sender发送一个TimeOut会话
  - receiver：localSet中作为常驻的receiver

  作用：

  服务端分配一系列指令的前置动作，也就是distributer和websocket Stream沟通的桥梁

  - sender：
    - 负责大部分任务分配
    - 发送特殊指令：超时
  - receiver：接收对应的数据，模式匹配会话以分配任务

  **UserSessions：**用户分片的主要功能是保存着一系列包括连接、命令结果在内的指令，必须显式地包含uid和device_id，这是隔离用户与设备的关键信息

  ```rust
  enum UserSessions{
      // Join: uid, device_id, WebsocketSessionsSender
      Join(String, String, WebsocketSessionsSender<WebsocketSessions>),
      // Disconnect: uid, device_id
      Disconnect(String, String),
      // Outcome: Outcome, recv_list
      Outcome(Outcome, Vec<String>),
      // ACK: uid, device_id, trace_id
     	ACK(String, String, i64),
      // TimeOut: uid, device_id, trace_id
      TimeOut(String, String, i64),
      // Ping
      Ping,
      // Pong: uid, PongMsg
      Pong(String, Vec<u8>)
  }
  ```

- WebsocketSessionsChannel（WebSocket会话通道）

  通道类型：WebsocketSessions

  作用：

  服务端处理一系列指令的后续动作，也就是processor与Websocket Sink 沟通的桥梁

  - sender：发送Command执行的结果Outcome、心跳请求Ping、断线请求Disconnect
  - receiver：接收对应的数据，通过websocket Sink发送到websocket客户端

  **WebsocketSessions：**Websocket分片的主要功能是保存着websocket请求的处理结果以及断线和心跳这样的请求，这部分数据和控制中枢没有关系，是直接提供给websocket Sink的，每一个Sink对应一个用户，所以不需要包含uid和device_id

  ```rust
  enum WebsocketSessions{
      // Message: Outcome
      // 客户端发送的Command的执行结果Outcome
      Message(String),
      // Disconnect: disconnect_code, disconnect_reason
      // 断线请求
      Disconnect(u16, String),
      // Ping: ping
      // 心跳请求
      Ping(Vec<u8>)
  }
  ```





##### websocket通信

axum的路由机制提供了方便的提取器，只要实现了FromRequest的就是一个提取器，使用axum自带的WebsocketUpgrade提取器，可以很容易地建立websocket连接。WebsocketUpgrade提取器是以这样的方式将http 连接升级为 websocket的流形式连接的：

```rust
	/// Finalize upgrading the connection and call the provided callback with
    /// the stream.
    ///
    /// When using `WebSocketUpgrade`, the response produced by this method
    /// should be returned from the handler. See the [module docs](self) for an
    /// example.
    pub fn on_upgrade<F, Fut>(self, callback: F) -> Response
    where
        F: FnOnce(WebSocket) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        //在建立三次握手后websocket正式建立
        let on_upgrade = self.on_upgrade;
        let config = self.config;

        let protocol = self.protocol.clone();

        tokio::spawn(async move {
            let upgraded = on_upgrade.await.expect("connection upgrade failed");
            //之后on_upgrade在不执行握手的情况下将原始套接字转换为WebsocketStream
            let socket =
                WebSocketStream::from_raw_socket(upgraded, protocol::Role::Server, Some(config))
                    .await;
            let socket = WebSocket {
                inner: socket,
                protocol,
            };
            //我们通过socket流来进行介于客户端和服务端的数据发送和接收
            callback(socket).await;
        });
        ......
    }
```

现在有关websocket两端之间数据交互的内容都在这个闭包内进行

将websocketStream拆分成Stream和Sink，Sink有点类似于发送数据缓冲区，Stream类似于接收数据缓冲区，他们通常是成对存在的，通过拆分，我们可以分别控制发送数据和接收数据的所有权，这样的处理方式可以使逻辑更加清晰。

拆分后的第一件事，应该是向控制中枢发送一个Join的会话，会话需要提供用户id，设备id，以及一个**WebsocketSessionsSender**

```rust
// Join: uid, device_id, WebsocketSessionsSender
Join(String, String, WebsocketSessionsSender)
```

这样用户登录就算完成了

###### 发送与接收

- Stream的接收

  生成一个新的异步任务，用于监听来自客户端的websocketStream数据，使用tokio::select!来监控接收到的数据。

  > **tokio::select!**用于监控异步表达式，和**golang::select**的不同之处在于后者主要监控channel，而前者用于监控async expression，不限于channel，也不是监控channel的可读可写状态。它的工作逻辑是：每次同时并发执行所有的分支的异步表达式，并在当前task上await它们，一旦有一个表达式以匹配模式的值完成，select！就返回这个分支的表达式的结果。

  ```rust
  Some(res) = Stream.next() => {
      //客户端发送的Message
  },
  _ = disconnect_rx => {
      //客户端主动发送过来的断开连接的请求
  }
  ```

  其中Message::Text就是Command，每执行一个Command都会得到一个Output，然后由UserSessionsSender发送

- Sink的发送

  在闭包中监控WebsocketSessionsChannel，Sink的发送依赖于**WebsocketSessionsChannel**的活跃程度。上文提到Join会话，控制中枢获取到Join会话的数据，保存了这个用户的WebsocketSessionsSender，这个sender发送WebsocketSessions会话到Websocket











### 通讯范式

#### command

The name of the command must be strictly bound to the action, not a pure noun or a noun modified with an adjective. E.g:

> ~~GroupMember~~​​ :x:
>
> GetGroupMember :heavy_check_mark:
>
> ***
>
> NewMessage :x:
>
> SendMessage :heavy_check_mark:

```rust
enum Command{
    // ACK: trace_id
    ACK(i64),
    // CreateUser: trace_id, username
    CreateUser(i64, String),
    
    
    // SendMessage: trace_id, message_id, org_id, stream, topic, sender, receiver, typ, content, timestamp, recv_list
    SendMessage(i64, String, String, String, String, String, String, String, String, String, Vec<String>),
    
    // SetTopicStatus: trace_id, stream, topic, status
    SetTopicStatus(i64, String, String, String, String),
    
    // SetReviewer: trace_id, stream, reviewer
    AppointReviewer(i64, String, String),
    // DismissReviewer: trace_id, stream, reviewer
    DismissReviewer(i64, String, String),
    
    // CreateGroup: trace_id, group_name, description
    CreateGroup(i64, String, String),
    // DismissGroup: trace_id, group_id
    DismissGroup(i64, String),
    
    // AddGroupMember: trace_id, group_id, uid
    AddGroupMember(i64, String, String),
    // DismissMember: trace_id, group_id, uid
    DismissMember(i64, String, String),
    
    // Vote: trace_id, stream, topic, votes
    Vote(i64, String, String, i8),   
    
    // Err: trace_id, error
    Err(i64, String)
}
```

#### Outcome

Possible Outcome: 

- Changes in messages
- Changes in unread messages
- Changes in topic status
- Changes in group and  membership
- Changes in visibility of stream
- Changes in reviewers list of  stream
- Changes in voting by reviewers of stream
- Changes in user data



结果（Outcome）的命名有两点要求：

1、与命令一一对应，每个命令必须要有对应的任务，这表达了命令执行的结果是成功还是失败。

2、命令执行成功或者失败后，可能会产生的影响，如“来新消息了”、“组加入新成员了”、“审核结果出炉”等，表达一种状态的变更。主要以形容词、副词加名词的形式命名。

```rust
enum Outcome{
    // ACK: trace_id
    ACK(i64)
    // CreateUser: trace_id, ok
    CreateUser(i64, bool),
    // UserCreated: trace_id, uid, username
    UserCreated(i64, String, String)
    
    // 消息变动
    // SendMessage: trace_id, ok, stream, topic, MessageData
    SendMessage(i64, bool, String, String, String)
    
    // 话题变动
    // SetTopicStatus: trace_id, ok, reviewer, stream, topic, status 
    SetTopicStatus(i64, bool, String, String, String, i8),
    
    // AppointReviewer: trace_id, ok
    AppointReviewer(i64, bool, String, String),
    // NewReviewer: trace_id, reviewer, stream
    NewReviewer(i64, String, String),
    // DismissReviewer: trace_id, ok
    DismissReviewer(i64, bool, String, String),
    // ReviewerDismissed: trace_id, reviewer, stream
    ReviewerDismissed(i64, String, String),
    
    // CreateGroup: trace_id, ok
    CreateGroup(i64, bool),
    // NewGroup: trace_id, group_id, group_name, description, role(TBD)
    NewGroup(i64, String, String, String, i8),
    // DismissGroup: trace_id, ok
    DismissGroup(i64, bool),
    // GroupDissolved: trace_id, group_id
    GroupDissolved(i64, String),
    
    // AddMember: trace_id, ok
    AddMember(i64, bool),
    // NewMember: trace_id, group_id, uid
    NewMember(i64, String, String),
    // DismissMember: trace_id, ok
    DismissMember(i64, bool),
    
    
    
    // Vote: trace_id, ok
    Vote(i64, bool, String, String, String, i8),
    // VoteUpdated: trace_id, stream, topic, votesList
    VoteUpdated(i64, String, String, HashMap<String, i8>),
    
    // TimeOut: trace_id
    TimeOut(i64),
    // Err: trace_id, error
    Err(i64, String)
    
}
```



# Drawbacks

[drawbacks]: #drawbacks

None at this time.

# Rationale and alternatives

[rationale-and-alternatives]: #rationale-and-alternatives

### Rationale

This design abstracts each operation into a task, and developers don't  have to worry about adding new operations in the future. They only need  to add new items to the enum and then execute them. The postback of the response has nothing to do with the core either. it's just  responsible for distribution.

### Alternatives

- Maybe use TCP or UDP instead of Websocket connection.
- sse is not necessary, you can let the connected client notify the front-end by itself through the channel
- 关于服务端消息通知，可以不采用sse，而是在前端与通讯客户端之间建立channel，客户端通过channel发送通知到前端。这种方案减少一种单工通信连接，
- Regarding server-side message notification, instead of using sse, a  channel can be established between the front-end and the communication  client, and the client can send notifications to the front-end through  the channel.

# Prior art

[prior-art]: #prior-art

The author is not aware of any prior art on this

# Unresolved questions

[unresolved-questions]: #unresolved-questions

None at this time.

# Future possibilities

[future-possibilities]: #future-possibilities

None at this time.