#  Slep API & Event

Author: wenjing
Data: 2022-12-12
Version: 0.0.1

[TOC]

## HTTP API
*地址：k8s.1to2to3.cn:31000*

### register

> 注册

请求地址: Post /register

- Request

  请求参数：

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | id   | 用户id | String |      | 否       |
  | name | 用户名 | String |      | 是       |

  - 示例

    ```
    localhost:8080/register
    ```

- Response

  返回参数：

  | 字段   | 说明     | 类型   | 备注      | 是否必填 |
  | ------ | -------- | ------ | --------- | -------- |
  | status | 状态码   | Int    | 成功: 200 | 是       |
  | data   | 正确结果 | String |           | 否       |
  | error  | 错误结果 | String |           | 否       |
  
  - 示例
  
    ```json
    # OK
    {
        "data": "register successful"
    }
    # Failed
    {
        "error": "error returned from database: duplicate key value violates unique constraint \"slep_user_pkey\""
    }
    ```
  
    

## Tauri Api

### Command

#### User

##### 登录：login

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | uid  | 用户id | String |      | 是       |

  - 示例

    ```react
    invoke('login', {
          uid: '123123',
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    UserNotExist: "HTTP error: 401 Unauthorized"
    ```


##### 登出：log_out

- Request

  请求参数:

  无

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```



##### 用户重命名：rename_username

- Request

  请求参数:

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | name | 用户名称 | String |      | 是       |

  - 示例

    ```react
    invoke('rename_username', {
          name: '123123',
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```


##### 获取用户信息：get_user_info

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | uid  | 用户id | String |      | 是       |

  - 示例

    ```
    invoke('get_user_info', {
          uid: '123123',
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```
    [{
    	"name": "asdasd",
    }]
    ```

#### Message

##### 发送消息：send_message

发送到stream前，必须先创建stream

- Request

  请求参数:

  | 字段        | 说明       | 类型          | 备注                                                         | 是否必填 |
  | ----------- | ---------- | ------------- | ------------------------------------------------------------ | -------- |
  | gid         | 组id       | i64           | 有值群聊，不填私聊                                           | 否       |
  | stream      | 流名称     | string        | 群聊必填                                                     | 否       |
  | topic       | 话题名称   | string        |                                                              | 是       |
  | messageType | 消息类型   | String        | unknown：未知, md：markdown, img：图片, video：视频, audio：音频 | 是       |
  | content     | 消息内容   | string        |                                                              | 是       |
  | sender      | 消息发送者 | i64           |                                                              | 是       |
  | receiver    | 消息接收者 | Option\<i64\> | 私聊必填                                                     | 否       |
  
- 示例
  
  - 流
    
    ```react
    invoke('send_message', {
      	gid: "7032557059040358394",
    	stream: "qwj_stream",
    	topic: "qwj_topic",
    	messageType: "md",
    	content: "啊啊啊啊啊",
    	sender: "1606174953750073345",
    })
    ```
    
    - 私聊
    
    ```react
    invoke('send_message', {
          topic: "private_chat",
          messageType: "md",
          content: "私聊啊啊啊",
          sender: "1606174953750073345",
          receiver: "1605822843254673409"
    })
    ```
    
    
  
- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  



##### 撤回消息：revoke_message（暂时不要用）

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | id   | 消息id | String |      | 是       |

  - 示例

    ```react
        invoke('revoke_message', {
          id: "7032592872830676986",
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```



##### 获取私聊消息：get_private_message

- Request

  请求参数:

  | 字段 | 说明       | 类型   | 备注 | 是否必填 |
  | ---- | ---------- | ------ | ---- | -------- |
  | uid  | 私聊对象id | string |      | 是       |

  - 示例

    ```react
    invoke('get_private_message', {
      sender: '1605104064619024400'
    })
    ```

- Response

  返回参数：

  | 字段     | 说明         | 类型   | 备注                                                         | 是否必填 |
  | -------- | ------------ | ------ | ------------------------------------------------------------ | -------- |
  | id       | 消息id       | i64    |                                                              | 是       |
  | typ      | 消息类型     | String | unknown：未知, md：markdown, img：图片, video：视频, audio：音频 | 是       |
  | addr_typ | 消息地址类型 | String | unknown：未知, private：私聊, stream：流                     | 是       |
  | topic    | 话题         | String |                                                              | 是       |
  | content  | 内容         | String |                                                              | 是       |
  | sender   | 发送者       | i64    |                                                              | 是       |
  | receiver | 接收者       | i64    |                                                              | 否       |

  - 示例

    ```json
    [{
    	"id": 1605498696370294784,
    	"typ": "md",
    	"addr_typ": "private",
    	"topic": "test topic",
    	"content": "test content",
    	"sender": 1605104064619024400,
    	"receiver": 1600441329433776128
    }, {
    	"id": 1605529320586022912,
    	"typ": "md",
    	"addr_typ": "private",
    	"topic": "test topic",
    	"content": "test content",
    	"sender": 1111,
    	"receiver": 1600441329433776128
    }]
    ```

##### 获取流消息：get_stream_message

- Request

  请求参数:

  | 字段   | 说明   | 类型   | 备注 | 是否必填 |
  | ------ | ------ | ------ | ---- | -------- |
  | gid    | 组id   | i64    |      | 是       |
  | stream | 流名称 | string |      | 是       |

  - 示例

    ```react
    invoke('get_stream_message', {
      gid: '1603716129428541441',
      stream: 'test stream',
    })
    ```

- Response

  返回参数：

  | 字段     | 说明         | 类型   | 备注                                                         | 是否必填 |
  | -------- | ------------ | ------ | ------------------------------------------------------------ | -------- |
  | id       | 消息id       | i64    |                                                              | 是       |
  | typ      | 消息类型     | String | unknown：未知, md：markdown, img：图片, video：视频, audio：音频 | 是       |
  | addr_typ | 消息地址类型 | String | unknown：未知, private：私聊, stream：流                     | 是       |
  | topic    | 话题         | String |                                                              | 是       |
  | content  | 内容         | String |                                                              | 是       |
  | sender   | 发送者       | i64    |                                                              | 是       |
  | receiver | 接收者       | i64    |                                                              | 否       |

  - 示例

    ```json
    [{
    	"id": 1605462705905799168,
    	"typ": "md",
    	"addr_typ": "stream",
    	"topic": "test topic",
    	"content": "test content",
    	"sender": 1600441329433776128,
    	"receiver": null
    }]
    ```

##### 获取话题消息：get_topic_message

- Request

  请求参数:

  | 字段   | 说明     | 类型   | 备注 | 是否必填 |
  | ------ | -------- | ------ | ---- | -------- |
  | gid    | 组id     | i64    |      | 是       |
  | stream | 流名称   | string |      | 是       |
  | topic  | 话题名称 | string |      | 是       |

  - 示例

    ```react
    invoke('get_topic_message', {
      gid: '1603716129428541441',
      stream: 'test stream',
      topic: 'test topic',
    })
    ```

- Response

  返回参数：

  | 字段     | 说明         | 类型   | 备注                                                         | 是否必填 |
  | -------- | ------------ | ------ | ------------------------------------------------------------ | -------- |
  | id       | 消息id       | i64    |                                                              | 是       |
  | typ      | 消息类型     | String | unknown：未知, md：markdown, img：图片, video：视频, audio：音频 | 是       |
  | addr_typ | 消息地址类型 | String | unknown：未知, private：私聊, stream：流                     | 是       |
  | topic    | 话题         | String |                                                              | 是       |
  | content  | 内容         | String |                                                              | 是       |
  | sender   | 发送者       | i64    |                                                              | 是       |
  | receiver | 接收者       | i64    |                                                              | 否       |

  - 示例

    ```json
    [{
    	"id": 1605462705905799168,
    	"typ": "md",
    	"addr_typ": "stream",
    	"topic": "test topic",
    	"content": "test content",
    	"sender": 1600441329433776128,
    	"receiver": null
    }]
    ```



#### UnreadStatus

##### 读消息：read

消息设置为已读

- Request

  请求参数:

  | 字段              | 说明       | 类型   | 备注 | 是否必填 |
  | ----------------- | ---------- | ------ | ---- | -------- |
  | addr              | 消息地址   | Addr   |      | 是       |
  | latest_message_id | 最新消息id | string |      | 是       |

  Addr

  | 字段      | 说明     | 类型     | 备注     | 是否必填 |
  | --------- | -------- | -------- | -------- | -------- |
  | uid       | 用户id   | i64      |          | 是       |
  | addr_type | 地址类型 | AddrType | 地址类型 | 是       |
  | topic     | 话题     | string   |          | 是       |

- 示例

  - 流

    ```react
    invoke('send_message', {
      	gid: "7032557059040358394",
    	stream: "qwj_stream",
    	topic: "qwj_topic",
    	messageType: "md",
    	content: "啊啊啊啊啊",
    	sender: "1606174953750073345",
    })
    ```

    - 私聊

    ```react
    invoke('send_message', {
          topic: "private_chat",
          messageType: "md",
          content: "私聊啊啊啊",
          sender: "1606174953750073345",
          receiver: "1605822843254673409"
    })
    ```

    

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```



#### Group

##### 创建组：create_group

- Request

  请求参数:

  | 字段       | 说明   | 类型   | 备注 | 是否必填 |
  | ---------- | ------ | ------ | ---- | -------- |
  | pid        | 父组id | i64    |      | 是       |
  | group_name | 组名称 | String |      | 是       |
  | des        | 描述   | string |      | 是       |

  - 示例

    ```react
    invoke('create_group', {
      pid: null,
      groupName: 'test',
      des: 'test group',
    })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  



##### 解散组：dismiss_group

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | uid  | 成员id | String |      | 是       |
  | gid  | 组id   | String |      | 是       |

  - 示例

    ```react
    invoke('dismiss_group', {
          gid: 1,
          uid: 1,
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  



##### 更新组信息：update_group_info

- Request

  请求参数:

  | 字段       | 说明   | 类型   | 备注 | 是否必填 |
  | ---------- | ------ | ------ | ---- | -------- |
  | gid        | 组id   | String |      | 是       |
  | pid        | 父组id | i64    |      | 否       |
  | group_name | 组名称 | String |      | 是       |
  | des        | 描述   | string |      | 是       |

  - 示例

    ```react
    invoke('update_group_info', {
          gid: "7032557059040358394",
          groupName: "qwj_dsss",
          des: "666"
    })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```

##### 获取下级组：get_sub_group

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | pid  | 父组id | String |      | 是       |

  - 示例

    ```react
        invoke('get_sub_group', {
          pid: "7032291334157512698",
        })
    ```

- Response

  返回参数：

  | 字段  | 说明 | 类型 | 备注 | 是否必填 |
  | ----- | ---- | ---- | ---- | -------- |
  | gid   | 组id | i64  |      | 是       |
  | level | 权限 | i16  |      |          |

  - 示例

    ```json
    [
        {
            "id": 7032596621024309235,
            "pid": 7032291334157512698,
            "name": "qwj_group",
            "des": "test"
        }
    ]
    ```



##### 通过uid查找组：get_group_by_uid

- Request

  请求参数:

  | 字段 | 说明   | 类型 | 备注 | 是否必填 |
  | ---- | ------ | ---- | ---- | -------- |
  | uid  | 用户id | i64  |      | 是       |

  - 示例

    ```react
        invoke('get_group_by_uid', {
          uid: '1600441329433776128',
        })
    ```

- Response

  返回参数：

  | 字段  | 说明 | 类型 | 备注 | 是否必填 |
  | ----- | ---- | ---- | ---- | -------- |
  | gid   | 组id | i64  |      | 是       |
  | level | 权限 | i16  |      |          |

  - 示例

    ```json
    [{
    	"gid": 1604670904116252673,
    	"level": 1
    }, {
    	"gid": 1605123490053165057,
    	"level": 3
    }]
    ```



##### 获取组员列表：get_group_member_list

- Request

  请求参数:

  | 字段 | 说明 | 类型   | 备注 | 是否必填 |
  | ---- | ---- | ------ | ---- | -------- |
  | gid  | 组id | String |      | 是       |

  - 示例

    ```react
        invoke('get_group_member_list', {
          gid: '1605123490053165057',
        })
    ```

- Response

  返回参数：

  | 字段  | 说明   | 类型 | 备注 | 是否必填 |
  | ----- | ------ | ---- | ---- | -------- |
  | gid   | 组id   | i64  |      | 是       |
  | level | 权限   | i16  |      |          |
  | uid   | 成员id | i64  |      |          |

  - 示例

    ```json
    [{
    	"gid": 1605123490053165057,
    	"uid": 1600441329433776128,
    	"level": 3
    }, {
    	"gid": 1605123490053165057,
    	"uid": 1605104064619024400,
    	"level": 1
    }]
    ```







#### Member

##### 添加成员：add_member

- Request

  请求参数:

  | 字段  | 说明     | 类型   | 备注 | 是否必填 |
  | ----- | -------- | ------ | ---- | -------- |
  | gid   | 组id     | String |      | 是       |
  | uid   | 成员id   | String |      | 是       |
  | level | 成员等级 | i16    |      | 是       |

  - 示例

    ```react
    invoke('add_member', {
          gid: 1,
          uid: 1,
          level: 1,
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  
    

##### 移除成员：dismiss_member

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | uid  | 成员id | String |      | 是       |
  | gid  | 组id   | String |      | 是       |

  - 示例

    ```react
    invoke('dismiss_member', {
          gid: 1,
          uid: 1,
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  

##### 更新成员信息：update_group_member_info

- Request

  请求参数:

  | 字段  | 说明   | 类型   | 备注 | 是否必填 |
  | ----- | ------ | ------ | ---- | -------- |
  | uid   | 用户id | String |      | 是       |
  | gid   | 组id   | String |      | 是       |
  | level | 身份   | i8     |      | 是       |

  - 示例

    ```react
    invoke('update_group_member_info', {
          uid: "1606174953750073345",
          gid: "7032557059040358394",
          level: 2
    })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```



##### 获取私聊成员列表：get_private_chat_list

- Request

  请求参数:

  | 字段 | 说明   | 类型   | 备注 | 是否必填 |
  | ---- | ------ | ------ | ---- | -------- |
  | uid  | 用户id | String |      | 是       |

  - 示例

    ```react
        invoke('get_private_chat_list', {
          uid: '1600441329433776128',
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型     | 备注 | 是否必填 |
  | ---- | -------- | -------- | ---- | -------- |
  | 无   | 成员列表 | Vec<i64> |      | 是       |

  - 示例

    ```json
    [1605104064619024400,1111]
    ```

    



#### Stream

##### 更新流设置：update_stream_settings

- Request

  请求参数:

  | 字段    | 说明     | 类型   | 备注 | 是否必填 |
  | ------- | -------- | ------ | ---- | -------- |
  | gid     | 组id     | i64    |      | 是       |
  | stream  | 流名称   | String |      | 是       |
  | des     | 流描述   | String |      | 是       |
  | r_level | 只读等级 | i16    |      | 是       |
  | w_level | 读写等级 | i16    |      | 是       |

  - 示例

    ```react
    invoke('update_stream_settings', {
          gid: 1,
          stream: 'test',
          des: 'test',
          r_level: 1,
          w_level: 2,
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  



##### 获取流列表：get_stream_list

- Request

  请求参数:

  | 字段 | 说明 | 类型   | 备注 | 是否必填 |
  | ---- | ---- | ------ | ---- | -------- |
  | gid  | 组id | String |      | 是       |

  - 示例

    ```react
    invoke('get_stream_settings', {
      gid: '1605123490053165057',
    })
    ```

- Response

  返回参数：

  | 字段   | 说明   | 类型   | 备注 | 是否必填 |
  | ------ | ------ | ------ | ---- | -------- |
  | stream | 流名称 | String |      | 是       |
  | des    | 描述   | String |      | 否       |
  | rlevel | 读权限 | i16    |      | 是       |
  | wlevel | 写权限 | i16    |      | 是       |

  - 示例

    ```json
    [{
    	"stream": "test stream",
    	"des": null,
    	"rlevel": 3,
    	"wlevel": 3
    }]
    ```





#### Topic

##### 更新话题设置：update_topic_settings（暂时不能用）

- Request

  请求参数:

  | 字段              | 说明     | 类型   | 备注 | 是否必填 |
  | ----------------- | -------- | ------ | ---- | -------- |
  | gid               | 组id     | i64    |      | 是       |
  | stream            | 流名称   | String |      | 是       |
  | topic             | 话题     | String |      | 是       |
  | associate_task_id | 任务id   | i64    |      | 是       |
  | r_level           | 只读等级 | i16    |      | 是       |
  | w_level           | 读写等级 | i16    |      | 是       |

  - 示例

    ```react
    invoke('update_topic_settings', {
          gid: 1,
          stream: 'test',
          topic: 'test',
          associate_task_id: 1,
          r_level: 1,
          w_level: 2,
        })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  
    

##### 获取话题设置：get_topic_settings

- Request

  请求参数:

  | 字段   | 说明   | 类型   | 备注 | 是否必填 |
  | ------ | ------ | ------ | ---- | -------- |
  | gid    | 组id   | i64    |      | 是       |
  | stream | 流名称 | String |      | 是       |
  | topic  | 话题   | String |      | 是       |

  - 示例

    ```react
    invoke('get_topic_settings', {
          gid: 1,
          stream: 'test',
          topic: 'test',
        })
    ```

- Response

  返回参数：

  | 字段              | 说明   | 类型 | 备注 | 是否必填 |
  | ----------------- | ------ | ---- | ---- | -------- |
  | associate_task_id | 任务id | i64  |      | 否       |
  | rlevel            | 读权限 | i16  |      | 是       |
  | wlevel            | 写权限 | i16  |      | 是       |

  - 示例

    ```json
    {"associate_task_id":null,"rlevel":3,"wlevel":3}
    ```

##### 获取话题列表：get_topic_list

- Request

  请求参数:

  | 字段     | 说明     | 类型   | 备注 | 是否必填 |
  | -------- | -------- | ------ | ---- | -------- |
  | gid      | 组id     | i64    |      | 否       |
  | stream   | 流名称   | String |      | 否       |
  | receiver | 私聊对象 | String |      | 否       |

  - 示例

    ```react
    invoke('get_topic_settings', {
        gid: '7033387592414670839',
        stream: 'aaa',
    })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型        | 备注 | 是否必填 |
  | ---- | -------- | ----------- | ---- | -------- |
  | 无   | 话题列表 | Vec<String> |      | 是       |

  - 示例

    ```json
    ["bbb4","bbb123dsafs","adsaf","bbb"]
    ```



#### OfficeAutomationTask

##### 下达命令：assign_task

- Request

  请求参数:

  | 字段      | 说明     | 类型   | 备注                                              | 是否必填 |
  | --------- | -------- | ------ | ------------------------------------------------- | -------- |
  | gid       | 组id     | i64    |                                                   | 否       |
  | stream    | 流名称   | String |                                                   | 否       |
  | receiver  | 个人id   | String |                                                   | 否       |
  | topic     | 话题     | String |                                                   | 是       |
  | name      | 任务名称 | String |                                                   | 是       |
  | des       | 描述     | String |                                                   | 否       |
  | typ       | 任务类型 | String | unknown：未知, group：组级任务, private：个人任务 | 是       |
  | consignor | 委托人   | i64    |                                                   | 是       |
  | deadline  | 截止日期 | i64    |                                                   | 是       |

  - 示例

    ```react
    invoke('assign_task', {
          gid: "7032557059040358394",
          stream: "单元测试stream",
          topic: "单元测试topic",
          name: "单元测试任务",
          des: "qwj_task",
          typ: "group",
          consignor: "1606174953750073
          deadline: "3434343434",
    })
    ```

- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  
    

##### 添加任务回执：add_task_receipt

- Request

  请求参数:

  | 字段     | 说明     | 类型   | 备注                                                         | 是否必填 |
  | -------- | -------- | ------ | ------------------------------------------------------------ | -------- |
  | hashkey  | 回执地址 | String | hash[gid + (stream or uid) + topic]                          | 是       |
  | task_id  | 任务id   | String |                                                              | 是       |
  | executor | 执行人   | i64    |                                                              | 是       |
  | status   | 回执状态 | String | unknown：未知, created：创建, pending：待定, confirmed：确认, blocked：阻塞, failed：拒绝 | 是       |
  | des      | 回执描述 | i64    |                                                              | 否       |
  
  - 示例
  
  ```react
    invoke('add_task_receipt', {
      hashkey: "-7978900843280652265",
        taskId: "7032624447458914299
        executor: "qwj_task",
        status: "group",
        des: "1606174953750073345",
    })
  ```
  
- Response

  返回参数：

  | 字段 | 说明     | 类型   | 备注 | 是否必填 |
  | ---- | -------- | ------ | ---- | -------- |
  | 无   | 错误信息 | String |      | 是       |

  - 示例

    ```json
    System: "CHANNEL_SENDER is none"
    ```
  
    

##### 根据委托人查看任务列表：get_task_list_by_consignor

- Request

  请求参数:

  | 字段      | 说明   | 类型   | 备注 | 是否必填 |
  | --------- | ------ | ------ | ---- | -------- |
  | consignor | 流名称 | String |      | 是       |

  - 示例

    ```react
    invoke('get_task_list_by_consignor', {
      consignor: '1600441329433776128',
    })
    ```

- Response

  返回参数：

  | 字段           | 说明         | 类型         | 备注                                              | 是否必填 |
  | -------------- | ------------ | ------------ | ------------------------------------------------- | -------- |
  | id             | 任务id       | i64          |                                                   | 是       |
  | name           | 任务名       | String       |                                                   | 是       |
  | hashkey        | 回执地址     | String       | hash[gid + (stream or uid) + topic]               | 是       |
  | des            | 任务描述     | String       |                                                   | 否       |
  | typ            | 任务类型     | String       | unknown：未知, group：组级任务, private：个人任务 | 是       |
  | consignor      | 委托人       | i64          |                                                   | 是       |
  | deadline       | 截止日期     | i64          |                                                   | 是       |
  | task_timestamp | 任务修改时间 | i64          |                                                   | 是       |
  | receipts       | 回执列表     | Vec<Receipt> |                                                   | 否       |
  

Receipt

| 字段              | 说明       | 类型   | 备注 | 是否必填 |
| ----------------- | ---------- | ------ | ---- | -------- |
| executor          | 回执执行人 | i64    |      | 是       |
| status            | 回执状态   | String |      | 是       |
| receipt_des       | 回执描述   | String |      | 是       |
| receipt_timestamp | 回执时间   | i64    |      | 是       |

- 示例
  
    ```json
    {
        "7032615789706620922": {
            "id": 7032615789706620922,
            "name": "单元测试任务",
            "hashkey": -7978900843280652265,
            "task_des": "qwj_task",
            "typ": "group",
            "consignor": 1606174953750073345,
            "deadline": 3434343434,
            "task_timestamp": 1676706617709,
            "receipts": null
        },
        "7032624447458914299": {
            "id": 7032624447458914299,
            "name": "单元测试任务",
            "hashkey": -7978900843280652265,
            "task_des": "qwj_task",
            "typ": "group",
            "consignor": 1606174953750073345,
            "deadline": 3434343434,
            "task_timestamp": 1676708328119,
            "receipts": [
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "1606174953750073345",
                    "receipt_timestamp": 1676710180566
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务hui",
                    "receipt_timestamp": 1676710192371
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务回执",
                    "receipt_timestamp": 1676710197631
                }
            ]
        }
    }
    ```



##### 根据任务类型获取任务列表：get_task_list_by_typ

- Request

  请求参数:

  | 字段 | 说明     | 类型   | 备注                                           | 是否必填 |
  | ---- | -------- | ------ | ---------------------------------------------- | -------- |
  | typ  | 任务状态 | String | unknow：未知, group：组任务, private：私人任务 | 是       |

  - 示例

    ```react
    invoke('get_task_list_by_typ', {
      typ: 'group',
    })
    ```

- Response

  返回参数：

  | 字段           | 说明         | 类型         | 备注                                              | 是否必填 |
  | -------------- | ------------ | ------------ | ------------------------------------------------- | -------- |
  | id             | 任务id       | i64          |                                                   | 是       |
  | name           | 任务名       | String       |                                                   | 是       |
  | hashkey        | 回执地址     | String       | hash[gid + (stream or uid) + topic]               | 是       |
  | des            | 任务描述     | String       |                                                   | 否       |
  | typ            | 任务类型     | String       | unknown：未知, group：组级任务, private：个人任务 | 是       |
  | consignor      | 委托人       | i64          |                                                   | 是       |
  | deadline       | 截止日期     | i64          |                                                   | 是       |
  | task_timestamp | 任务修改时间 | i64          |                                                   | 是       |
  | receipts       | 回执列表     | Vec<Receipt> |                                                   | 否       |
  
  Receipt
  
  | 字段              | 说明       | 类型   | 备注 | 是否必填 |
  | ----------------- | ---------- | ------ | ---- | -------- |
  | executor          | 回执执行人 | i64    |      | 是       |
  | status            | 回执状态   | String |      | 是       |
  | receipt_des       | 回执描述   | String |      | 是       |
  | receipt_timestamp | 回执时间   | i64    |      | 是       |
  
  - 示例
  
    ```json
    {
        "7032615789706620922": {
            "id": 7032615789706620922,
            "name": "单元测试任务",
            "hashkey": -7978900843280652265,
            "task_des": "qwj_task",
            "typ": "group",
            "consignor": 1606174953750073345,
            "deadline": 3434343434,
            "task_timestamp": 1676706617709,
            "receipts": null
        },
        "7032624447458914299": {
            "id": 7032624447458914299,
            "name": "单元测试任务",
            "hashkey": -7978900843280652265,
            "task_des": "qwj_task",
            "typ": "group",
            "consignor": 1606174953750073345,
            "deadline": 3434343434,
            "task_timestamp": 1676708328119,
            "receipts": [
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "1606174953750073345",
                    "receipt_timestamp": 1676710180566
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务hui",
                    "receipt_timestamp": 1676710192371
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务回执",
                    "receipt_timestamp": 1676710197631
                }
            ]
        }
    }
    ```
  



##### 获取任务列表：get_task_list

- Request

  请求参数:

  无

  - 示例

    ```react
    invoke('get_task_list', {})
    ```

- Response

  返回参数：

  | 字段           | 说明         | 类型         | 备注                                              | 是否必填 |
  | -------------- | ------------ | ------------ | ------------------------------------------------- | -------- |
  | id             | 任务id       | i64          |                                                   | 是       |
  | name           | 任务名       | String       |                                                   | 是       |
  | hashkey        | 回执地址     | String       | hash[gid + (stream or uid) + topic]               | 是       |
  | des            | 任务描述     | String       |                                                   | 否       |
  | typ            | 任务类型     | String       | unknown：未知, group：组级任务, private：个人任务 | 是       |
  | consignor      | 委托人       | i64          |                                                   | 是       |
  | deadline       | 截止日期     | i64          |                                                   | 是       |
  | task_timestamp | 任务修改时间 | i64          |                                                   | 是       |
  | receipts       | 回执列表     | Vec<Receipt> |                                                   | 否       |

  Receipt

  | 字段              | 说明       | 类型   | 备注 | 是否必填 |
  | ----------------- | ---------- | ------ | ---- | -------- |
  | executor          | 回执执行人 | i64    |      | 是       |
  | status            | 回执状态   | String |      | 是       |
  | receipt_des       | 回执描述   | String |      | 是       |
  | receipt_timestamp | 回执时间   | i64    |      | 是       |

  - 示例

    ```json
    {
        "7032624447458914299": {
            "id": 7032624447458914299,
            "name": "单元测试任务",
            "hashkey": -7978900843280652265,
            "task_des": "qwj_task",
            "typ": "group",
            "consignor": 1606174953750073345,
            "deadline": 3434343434,
            "task_timestamp": 1676708328119,
            "receipts": [
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "1606174953750073345",
                    "receipt_timestamp": 1676710180566
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务hui",
                    "receipt_timestamp": 1676710192371
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务回执",
                    "receipt_timestamp": 1676710197631
                }
            ]
        }
    }
    ```

  





##### 获取任务信息：get_task_info

- Request

  请求参数:

  | 字段    | 说明   | 类型   | 备注 | 是否必填 |
  | ------- | ------ | ------ | ---- | -------- |
  | task_id | 任务id | String |      | 是       |

  - 示例

    ```react
    invoke('get_task_info', {
      task_id: '7032623836894081008',
    })
    ```

- Response

  返回参数：

  | 字段           | 说明         | 类型         | 备注                                              | 是否必填 |
  | -------------- | ------------ | ------------ | ------------------------------------------------- | -------- |
  | id             | 任务id       | i64          |                                                   | 是       |
  | name           | 任务名       | String       |                                                   | 是       |
  | hashkey        | 回执地址     | String       | hash[gid + (stream or uid) + topic]               | 是       |
  | des            | 任务描述     | String       |                                                   | 否       |
  | typ            | 任务类型     | String       | unknown：未知, group：组级任务, private：个人任务 | 是       |
  | consignor      | 委托人       | i64          |                                                   | 是       |
  | deadline       | 截止日期     | i64          |                                                   | 是       |
  | task_timestamp | 任务修改时间 | i64          |                                                   | 是       |
  | receipts       | 回执列表     | Vec<Receipt> |                                                   | 否       |
  
  Receipt
  
  | 字段              | 说明       | 类型   | 备注 | 是否必填 |
  | ----------------- | ---------- | ------ | ---- | -------- |
  | executor          | 回执执行人 | i64    |      | 是       |
  | status            | 回执状态   | String |      | 是       |
  | receipt_des       | 回执描述   | String |      | 是       |
  | receipt_timestamp | 回执时间   | i64    |      | 是       |
  
  - 示例
  
    ```json
    {
        "7032624447458914299": {
            "id": 7032624447458914299,
            "name": "单元测试任务",
            "hashkey": -7978900843280652265,
            "task_des": "qwj_task",
            "typ": "group",
            "consignor": 1606174953750073345,
            "deadline": 3434343434,
            "task_timestamp": 1676708328119,
            "receipts": [
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "1606174953750073345",
                    "receipt_timestamp": 1676710180566
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务hui",
                    "receipt_timestamp": 1676710192371
                },
                {
                    "executor": 1606174953750073345,
                    "status": "created",
                    "receipt_des": "任务回执",
                    "receipt_timestamp": 1676710197631
                }
            ]
        }
    }
    ```
  
  



## Event

### 数据结构：

Insert：插入

Upsert：插入，存在即更新

Update：更新

| 字段     | 说明         | 类型   | 备注 |
| -------- | ------------ | ------ | ---- |
| id       | 资源id       | i64    |      |
| resource | 具体数据结构 | String |      |

Drop：删除

| 字段 | 说明         | 类型 | 备注 |
| ---- | ------------ | ---- | ---- |
| id   | 删除的资源id | i64  |      |

### User

#### 重命名：RenameUsername

- Response

  返回参数：

  | 字段      | 说明     | 类型   | 备注 |
  | --------- | -------- | ------ | ---- |
  | id        | 用户id   | i64    |      |
  | name      | 用户名   | String |      |
  | timestamp | 修改时间 | i64    |      |

  **Message**：

  - 示例

    ```json
    {
        "Upsert": {
            "id": 1606174953750073345,
            "resource": {
                "name": "qwj04",
                "timestamp": 1676702108244
            }
        }
    }
    ```

    

#### 登录：Login

- Response

  返回参数：

  | 字段      | 说明     | 类型   | 备注 |
  | --------- | -------- | ------ | ---- |
  | id        | 用户id   | i64    |      |
  | name      | 用户名   | String |      |
  | timestamp | 修改时间 | i64    |      |

  **Message**：

  - 示例

    ```json
    {
        "Upsert": {
            "id": 1606174953750073345,
            "resource": {
                "name": "qwj04",
                "timestamp": 1676702108244
            }
        }
    }
    ```


#### 初始化：Initialize

- Response

  返回参数：

  无

### Message

#### 发送消息：SendMessage

- Response

  返回参数：

  | 字段      | 说明     | 类型    | 备注 |
  | --------- | -------- | ------- | ---- |
  | gid       | 组id     | i64     |      |
  | stream    | 流名称   | String  |      |
  | message   | 消息数据 | Message |      |
  | timestamp | 修改时间 | i64     |      |
  
  **Message**：

  | 字段      | 说明     | 类型   | 备注                                                         |
  | --------- | -------- | ------ | ------------------------------------------------------------ |
  | id        | trace_id | i64    |                                                              |
  | typ       | 消息类型 | String | unknown：未知, md：markdown, img：图片, video：视频, audio：音频 |
  | gid       | 组id     | String |                                                              |
  | addr      | 消息地址 | String | uid：私聊对象id 或 stream： 流名称                           |
  | topic     | 话题     | String |                                                              |
  | content   | 内容     | String |                                                              |
  | sender    | 发送者   | i64    |                                                              |
  | timestamp | 修改时间 | i64    |                                                              |
  
  
  
- 示例
  
  - 流
  
  ```json
  {
      "Upsert": {
          "id": 7032592872830676986,
          "resource": {
              "gid": 7032557059040358394,
              "typ": "md",
              "addr": "qwj_stream",
              "topic": "qwj_topic",
              "content": "啊啊啊啊啊",
              "sender": 1606174953750073345,
              "timestamp": 1676700810427
          }
      }
  }
  ```
  
  - 私聊
  
  ```json
  {
      "Upsert": {
          "id": 7032596621024309242,
          "resource": {
              "gid": null,
              "typ": "md",
              "addr": "1605822843254673409",
              "topic": "private_chat",
              "content": "私聊啊啊啊",
              "sender": 1606174953750073345,
              "timestamp": 1676701765387
          }
      }
  }
  ```
  



### Group

#### 创建组：CreateGroup

- Response

  返回参数：

  | 字段      | 说明       | 类型   | 备注 |
  | --------- | ---------- | ------ | ---- |
  | id        | 组id       | i64    |      |
  | pid       | 父组id     | i64    |      |
  | name      | 组名称     | String |      |
  | des       | 描述       | String |      |
  | timestamp | 修改时间戳 | i64    |      |
  
  - 示例

    ```json
    {
        "Upsert": {
            "id": 7032286763418923002,
            "resource": {
                "pid": null,
                "name": "qwj_group",
                "des": "test",
                "timestamp": 1676627825905
            }
        }
    }
    ```



#### 解散组：DismissGroup

- Response

  返回参数：

  | 字段 | 说明 | 类型 | 备注 |
  | ---- | ---- | ---- | ---- |
  | 无   | 组id | i64  |      |

  - 示例

    ```json
    {
        "Drop": 7032291334157512698
    }
    ```

#### 更新组：UpdateGroupInfo

- Response

  返回参数：

  | 字段      | 说明       | 类型   | 备注 |
  | --------- | ---------- | ------ | ---- |
  | id        | 组id       | i64    | 是   |
  | pid       | 父组id     | i64    | 否   |
  | name      | 组名       | String | 是   |
  | des       | 组描述     | String | 是   |
  | timestamp | 修改时间戳 | i64    | 是   |

  - 示例

    ```json
    {
        "Upsert": {
            "id": 7032557059040358394,
            "resource": {
                "pid": null,
                "name": "qwj_dsss",
                "des": "666",
                "timestamp": 1676703943073
            }
        }
    }
    ```



### Member

#### 添加成员：AddMember

- Response

  返回参数：

  | 字段      | 说明         | 类型   | 备注                   |
  | --------- | ------------ | ------ | ---------------------- |
  | id        | 用户id, 组id | i64    | id[0]：uid/ id[1]：gid |
  | level     | 身份         | String |                        |
  | timestamp | 修改时间戳   | i64    |                        |
  
  - 示例
  
  ```json
    {
      "Upsert": {
            "id": [
                1606174953750073345,
                7032557059040358394
            ],
            "resource": {
                "level": 1,
                "timestamp": 1676692283074
            }
        }
    }
  ```





#### 移除成员：DismissMember

- Response

  返回参数：

  | 字段 | 说明         | 类型 | 备注                    |
  | ---- | ------------ | ---- | ----------------------- |
  | 无   | 用户id, 组id | i64  | id[0]：uid / id[1]：gid |

  - 示例

    ```json
    {
        "Drop": [
            1605822843254673409,
            7032557059040358394
        ]
    }
    ```

#### 更新组成员身份：UpdateGroupMemberInfo

- Response

  返回参数：

  | 字段      | 说明       | 类型 | 备注 |
  | --------- | ---------- | ---- | ---- |
  | id        | 用户id     | i64  | 是   |
  | level     | 身份       | i8   | 是   |
  | timestamp | 修改时间戳 | i64  | 是   |

  - 示例

    ```json
    {
        "Upsert": {
            "id": [
                1606174953750073345,
                7032557059040358394
            ],
            "resource": {
                "level": 2,
                "timestamp": 1676704969892
            }
        }
    }
    ```



### Stream

#### 更新流设置：UpdateStreamSettings

- Response

  返回参数：

  | 字段      | 说明         | 类型          | 备注                       |
  | --------- | ------------ | ------------- | -------------------------- |
  | id        | 组id，流名称 | [i64，String] | id[0]：gid / id[1]：stream |
  | des       | 流描述       | String        |                            |
  | rlevel    | 读权限       | i16           |                            |
  | wlevel    | 写权限       | i16           |                            |
  | timestamp | 修改时间     | i64           |                            |
  
  - 示例
  
  ```json
    {
      "Upsert": {
            "id": [
                "qwj_stream",
                7032557059040358394
            ],
            "resource": {
                "des": "qwj_stream",
                "rlevel": 1,
                "wlevel": 2,
                "timestamp": 1676693214616
            }
        }
    }
  ```



### Topic

#### 更新话题设置：UpdateTopicSettings（暂时不能用）

- Response

  返回参数：

  | 字段              | 说明     | 类型   | 备注 |
  | ----------------- | -------- | ------ | ---- |
  | trace_id          | trace_id | i64    |      |
  | action            | 操作     | String |      |
  | gid               | 组id     | i64    |      |
  | stream            | 流名称   | String |      |
  | topic             | 话题     | String |      |
  | associate_task_id | 任务id   | i64    |      |
  | rlevel            | 读权限   | i16    |      |
  | wlevel            | 写权限   | i16    |      |
  | timestamp         | 修改时间 | i64    |      |

  - 示例

    ```json
    {
    	"TopicSettings": {
    		"trace_id": 1605462705647456256,
    		"action": "Update",
    		"gid": 1605123490053165057,
    		"stream": "test stream",
    		"topic": "test topic",
    		"associate_task_id": null,
    		"rlevel": 3,
    		"wlevel": 3,
    		"timestamp": 1671607111527
    	}
    }
    ```

### OfficeAutomationTask

#### 下达OA任务：AssignTask

- Response

  返回参数：

  | 字段      | 说明     | 类型   | 备注                       |
  | --------- | -------- | ------ | -------------------------- |
  | id        | 任务id   | i64    |                            |
  | name      | 任务名   | String |                            |
  | des       | 任务描述 | i64    |                            |
  | typ       | 任务类型 | String | group：组 或 private：个人 |
  | consignor | 委托人   | i64    |                            |
  | deadline  | 截止日期 | i64    |                            |
  | timestamp | 修改时间 | i64    |                            |

  - 示例

    ```json
    {
        "Insert": {
            "id": 7032615789706620922,
            "resource": {
                "name": "单元测试任务",
                "des": "qwj_task",
                "typ": "group",
                "consignor": 1606174953750073345,
                "deadline": 3434343434,
                "timestamp": 1676706617709
            }
        }
    }
    ```

#### 添加任务到话题：AddTaskToTopic

- Response

  返回参数：

  | 字段              | 说明     | 类型   | 备注                     |
  | ----------------- | -------- | ------ | ------------------------ |
  | id                | hashkey  | i64    | hash(gid + addr + topic) |
  | associate_task_id | 任务id   | String |                          |
  | gid               | 组id     | i64    |                          |
  | addr              | 任务地址 | String | stream：组 或 uid：个人  |
  | topic             | 话题名   | String |                          |
  | timestamp         | 修改时间 | i64    |                          |

  - 示例

    ```json
    {
        "Upsert": {
            "id": -3396462162505099978,
            "resource": {
                "associate_task_id": 7032615789706620922,
                "gid": 7032557059040358394,
                "addr": "单元测试stream",
                "topic": "单元测试topic",
                "timestamp": 1676706617709
            }
        }
    }
    ```

#### 发送任务消息：SendTaskMessage

- Response

  返回参数：

  | 字段      | 说明     | 类型   | 备注                    |
  | --------- | -------- | ------ | ----------------------- |
  | id        | 消息id   | i64    |                         |
  | gid       | 组id     | i64    |                         |
  | typ       | 消息类型 | String | 固定为unknown           |
  | addr      | 任务地址 | String | stream：组 或 uid：个人 |
  | topic     | 话题名   | String |                         |
  | content   | 消息内容 | String |                         |
  | sender    | 发送方   | i64    | 固定为0                 |
  | timestamp | 修改时间 | i64    |                         |

  - 示例

    ```json
    {
        "Insert": {
            "id": 7032624447458914287,
            "resource": {
                "gid": 7032557059040358394,
                "typ": "unknown",
                "addr": "单元测试stream",
                "topic": "单元测试topic",
                "content": "发布任务",
                "sender": 0,
                "timestamp": 1676708748516
            }
        }
    }
    ```



### TaskReceipt

#### 添加任务回执：AddTaskReceipt

- Response

  返回参数：

  | 字段      | 说明     | 类型   | 备注                                                         |
  | --------- | -------- | ------ | ------------------------------------------------------------ |
  | id        | 任务id   | i64    |                                                              |
  | hashkey   | hashkey  | i64    | hash(gid + addr + topic)                                     |
  | executor  | 执行人   | i64    |                                                              |
  | status    | 回执状态 | String | unknown：未知, created：创建, pending：待定, confirmed：确认, blocked：阻塞, failed：拒绝 |
  | des       | 回执描述 | i64    |                                                              |
  | timestamp | 修改时间 | i64    |                                                              |

  - 示例

    ```json
    {
        "Update": {
            "id": 7032624447458914299,
            "resource": {
                "hashkey": -3396462162505099978,
                "executor": 1606174953750073345,
                "status": "created",
                "des": "任务回执",
                "timestamp": 1676710197631
            }
        }
    }
    ```

#### 发送任务回执消息：SendTaskReceiptMessage

- Response

  返回参数：

  | 字段      | 说明     | 类型   | 备注                    |
  | --------- | -------- | ------ | ----------------------- |
  | id        | 消息id   | i64    |                         |
  | gid       | 组id     | i64    |                         |
  | typ       | 消息类型 | String | 固定为unknown           |
  | addr      | 任务地址 | String | stream：组 或 uid：个人 |
  | topic     | 话题名   | String |                         |
  | content   | 消息内容 | String |                         |
  | sender    | 发送方   | i64    | 固定为0                 |
  | timestamp | 修改时间 | i64    |                         |

  - 示例

    ```json
    {
        "Insert": {
            "id": 7032624447458914287,
            "resource": {
                "gid": 7032557059040358394,
                "typ": "unknown",
                "addr": "单元测试stream",
                "topic": "单元测试topic",
                "content": "发布任务",
                "sender": 0,
                "timestamp": 1676708748516
            }
        }
    }
    ```

