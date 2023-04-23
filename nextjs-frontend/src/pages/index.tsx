import BottomControl from '@/components/BottomControl';
import Layout from '@/components/Layout';
import MessageViewer from '@/components/MessageViewer';
import PrivateMesssagesPartBuilder from '@/components/PrivateMesssagesPartBuilder';
import StreamInfo from '@/components/StreamInfo';
import StreamsPartBuilder from '@/components/StreamsPartBuilder';
import TopicInfo from '@/components/TopicInfo';
import styles from '@/styles/myStyles.module.scss';
import { indexContext } from '@/utils/createContext';
import { listen } from '@tauri-apps/api/event';
import classNames from 'classnames';
import { ReactElement, useEffect, useRef, useState } from 'react';
import { NextPageWithLayout } from './_app';

//事件的消息体
export interface Payload {
  message: Array<string>;
  timestamp: number;
}
export type conversationsSelectedType = {
  body?: string | Record<string, any>;
  type?: 'private message' | 'stream' | 'topic';
};

const index: NextPageWithLayout = () => {
  const [conversationsSelected, setconversationsSelected] =
    useState<conversationsSelectedType>({}); //选中对话
  const [topicComboboxSelected, settopicComboboxSelected] =
    useState<conversationsSelectedType>({}); //topic自动完成框选中的topic
  const [streamSelected, setstreamSelected] = useState<
    Record<string, any> | undefined
  >(undefined); //选中的stream
  const [streamComboboxSelected, setstreamComboboxSelected] = useState<
    Record<string, any> | undefined
  >(undefined); //stream自动完成框选中的stream
  const [editorByButtonType, seteditorByButtonType] = useState<
    undefined | 'default' | 'topic' | 'private message'
  >(undefined);
  const messageBodyFocusFlag = useRef(false);
  const [CreateStreamVisible, setCreateStreamVisible] = useState(false);

  // useEffect(() => {
  //   console.error('conversationsSelected', conversationsSelected);
  // }, [conversationsSelected]);
  // useEffect(() => {
  //   console.error('streamSelected', streamSelected);
  // }, [streamSelected]);
  // useEffect(() => {
  //   console.error('streamComboboxSelected', streamComboboxSelected);
  // }, [streamComboboxSelected]);
  // useEffect(() => {
  //   console.error('topicComboboxSelected', topicComboboxSelected);
  // }, [topicComboboxSelected]);

  //======================= 后端测试 =========================

  // useEffect(() => {
  //   invoke('dismiss_reviewer', {
  //     uid: "1614539594746236928",
  //     gid: "7020653969047564283",
  //   }).then(res => {
  //     console.log('res', res);
  //   });
  //   ;
  // }, []);

  // useEffect(() => {
  //   invoke('appoint_reviewer', {
  //     uid: "1614539594746236928",
  //     gid: "7020653969047564283",
  //   }).then(res => {
  //     console.log('res', res);
  //   });
  //   ;
  // }, []);
  // useEffect(() => {
  //   invoke('login', {
  //     uid: "1605822843254673409",
  //     uuid: "qwj",
  //   }).then(res => {
  //     console.log('res', res);
  //   });
  // }, []);

  useEffect(() => {
    // invoke('greet', {
    //   id: 1606174953750073345,
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('create_group', {
    //   uid: "1606174953750073345",
    //   pid: "7032291334157512698",
    //   groupName: "qwj_group",
    //   des: "test",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('get_sub_group', {
    //   pid: "7032291334157512698",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('get_group_by_uid', {
    //   uid: "1606174953750073345",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('get_group_member_list', {
    //   gid: "7032291334157512698",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('dismiss_group', {
    //   uid: "1606174953750073345",
    //   gid: "7032291334157512698",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('rename_username', {
    //   uid: "1606174953750073345",
    //   name: "qwj04",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('add_member', {
    //   uid: "1605822843254673409",
    //   gid: "7032557059040358394",
    //   level: 3
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('dismiss_member', {
    //   uid: "1605822843254673409",
    //   gid: "7032557059040358394",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('update_stream_settings', {
    //   gid: "7032557059040358394",
    //   stream: "单元测试stream",
    //   des: "啊实打实的",
    //   rlevel: 1,
    //   wlevel: 2,
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('send_message', {
    //   gid: "7032557059040358394",
    //   stream: "qwj_stream",
    //   topic: "qwj_topic",
    //   messageType: "md",
    //   content: "啊啊啊啊啊",
    //   sender: "1606174953750073345",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('send_message', {
    //   topic: "private_chat",
    //   messageType: "md",
    //   content: "私聊啊啊啊",
    //   sender: "1606174953750073345",
    //   receiver: "1605822843254673409"
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('revoke_message', {
    //   id: "7032592872830676986",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('update_group_member_info', {
    //   uid: "1606174953750073345",
    //   gid: "7032557059040358394",
    //   level: 2
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('update_group_info', {
    //   gid: "7032557059040358394",
    //   groupName: "qwj_dsss",
    //   des: "666"
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('assign_task', {
    //   gid: "7032557059040358394",
    //   stream: "单元测试stream",
    //   topic: "单元测试topic",
    //   name: "单元测试任务",
    //   des: "qwj_task",
    //   typ: "group",
    //   consignor: "1606174953750073345",
    //   deadline: "3434343434",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('add_task_receipt', {
    //   gid: "7032557059040358394",
    //   stream: "单元测试stream",
    //   topic: "单元测试topic",
    //   taskId: "7032624447458914299",
    //   executor: "1606174953750073345",
    //   status: "created",
    //   des: "任务回执",
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('get_task_list_by_typ', {
    //    typ: 'group',
    // }).then(res => {
    //   console.log('res', res);
    // });
    //   invoke('get_task_list_by_consignor', {
    //     consignor: '1606174953750073345',
    //  }).then(res => {
    //    console.log('res', res);
    //  });
    // invoke('get_task_info', {
    //   taskId: '7032623836894081008',
    // }).then(res => {
    //   console.log('res', res);
    // });
    // invoke('get_topic_list', {
    // }).then(res => {
    //   console.log('res', res);
    // });
  }, []);

  //======================== 后端测试 ========================

  const Initialize = async () => {
    return await listen<Payload>('Initialize', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const create_group = async () => {
    return await listen<Payload>('CreateGroup', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const add_member = async () => {
    return await listen<Payload>('AddMember', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const send_message = async () => {
    return await listen<Payload>('SendMessage', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const update_stream_settings = async () => {
    return await listen<Payload>('UpdateStreamSettings', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const update_topic_settings = async () => {
    return await listen<Payload>('UpdateTopicSettings', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const assign_task = async () => {
    return await listen<Payload>('AssignTask', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const add_task_to_topic = async () => {
    return await listen<Payload>('AddTaskToTopic', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const send_task_message = async () => {
    return await listen<Payload>('SendTaskMessage', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const add_task_receipt = async () => {
    return await listen<Payload>('AddTaskReceipt', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const send_task_receipt_message = async () => {
    return await listen<Payload>('SendTaskReceiptMessage', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const add_stream_settings = async () => {
    return await listen<Payload>('AddStreamSettings', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const add_topic_settings = async () => {
    return await listen<Payload>('AddTopicSettings', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const complement_user_info = async () => {
    return await listen<Payload>('ComplementUserInfo', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const click = async () => {
    return await listen<Payload>('click', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const rename_username = async () => {
    return await listen<Payload>('RenameUserInfo', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };
  const login = async () => {
    return await listen<Payload>('Login', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
    });
  };

  useEffect(() => {
    Initialize();
    complement_user_info();
    create_group();
    add_member();
    send_message();
    update_stream_settings();
    update_topic_settings();
    assign_task();
    add_task_to_topic();
    send_task_message();
    add_task_receipt();
    send_task_receipt_message();
    add_stream_settings();
    add_topic_settings();
    click();
    rename_username();
    login();
  }, []);

  // console.log('first', stream_taskHander());

  return (
    <indexContext.Provider
      value={{
        conversationsSelected,
        setconversationsSelected,
        streamSelected,
        setstreamSelected,
        topicComboboxSelected,
        settopicComboboxSelected,
        streamComboboxSelected,
        setstreamComboboxSelected,
        editorByButtonType,
        seteditorByButtonType,
        messageBodyFocusFlag,
        CreateStreamVisible,
        setCreateStreamVisible,
      }}
    >
      <div className={classNames('flex h-full ')}>
        <div
          className={classNames(
            'w-60 h-full shrink-0 bg-[#F5F6F7] rounded-tl-xl px-[10px] py-5 flex flex-col',
            styles.noScrollbar
          )}
        >
          <div className={classNames('text-xl pl-4 mb-8')}>消息</div>
          <PrivateMesssagesPartBuilder />
          <StreamsPartBuilder />
        </div>

        <div className={classNames('grow w-0 flex flex-col px-2')}>
          <MessageViewer />
          <BottomControl />
        </div>
        <div
          className={classNames(
            'hidden w-96 h-full shrink-0 bg-white px-[10px] py-5 xl:grid grid-rows-1  '
          )}
        >
          {conversationsSelected.type === 'topic' && <TopicInfo />}
          {conversationsSelected.type === 'stream' && <StreamInfo />}
        </div>
      </div>
    </indexContext.Provider>
  );
};

index.getLayout = function getLayout(page: ReactElement) {
  return <Layout>{page}</Layout>;
};

export default index;
