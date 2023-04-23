import { Payload } from '@/pages';
import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import '@toast-ui/editor-plugin-color-syntax/dist/toastui-editor-plugin-color-syntax.css';
import '@toast-ui/editor/dist/i18n/zh-cn';
import '@toast-ui/editor/dist/toastui-editor.css';
import { Editor } from '@toast-ui/react-editor';
import { useEventListener, useKeyPress } from 'ahooks';
import classNames from 'classnames';
import { Dispatch, SetStateAction, useContext, useEffect, useRef } from 'react';
import { useSWRConfig } from 'swr';

const MarkdownEditor = ({
  setstreamWarn,
}: {
  setstreamWarn: Dispatch<SetStateAction<boolean>>;
}) => {
  const { mutate } = useSWRConfig();
  const {
    topicComboboxSelected,
    streamComboboxSelected,
    conversationsSelected,
    streamSelected,
  } = useContext(indexContext);
  const topicComboboxSelectedRef = useRef(topicComboboxSelected);
  topicComboboxSelectedRef.current = topicComboboxSelected;
  const streamComboboxSelectedRef = useRef(streamComboboxSelected);
  streamComboboxSelectedRef.current = streamComboboxSelected;
  const conversationsSelectedRef = useRef(conversationsSelected);
  conversationsSelectedRef.current = conversationsSelected;
  const streamSelectedRef = useRef(streamSelected);
  streamSelectedRef.current = streamSelected;

  const editorRef = useRef<Editor>(null);
  const editorDivRef = useRef<HTMLDivElement>(null);

  const sendContent = () => {
    if (
      editorRef?.current &&
      editorRef.current?.getInstance().getMarkdown() !== ''
    ) {
      // 还未考虑私聊
      if (!!streamComboboxSelected?.new) {
        // invoke('update_stream_settings', {
        //   gid: getLocaleStorageUtil('gid'),
        //   stream: streamComboboxSelected.stream,
        //   des: '',
        //   rlevel: 1,
        //   wlevel: 1,
        // });
        setstreamWarn(true);
      } else {
        invoke('send_message', {
          gid: getLocaleStorageUtil('gid'),
          messageType: 'md',
          addrType: 'stream',
          stream: streamComboboxSelected.stream,
          topic: topicComboboxSelected.body,
          content: editorRef.current!.getInstance().getMarkdown(),
          sender: getLocaleStorageUtil('uid'),
        });
      }
    } else {
      console.error('You have nothing to send!');
    }
  };

  const createSendButton = () => {
    const button = document.createElement('button');

    button.className = 'toastui-editor-toolbar-icons first';
    button.style.backgroundImage = 'none';
    button.style.margin = '0';
    button.style.color = 'white';
    button.style.width = '60px';
    button.style.background = '#5f5ffb';
    button.style.borderRadius = '8px';
    button.style.fontWeight = '600';
    button.innerHTML = `<i>send</i>`;
    button.className = 'sendButton';
    // useContext访问不到
    // button.addEventListener('click', () => {
    //   console.log('conversationsSelected', conversationsSelected);
    //   sendContent();
    // });

    return button;
  };

  useEventListener(
    'click',
    () => {
      sendContent();
    },
    { target: document.querySelector('.sendButton') }
  );

  useKeyPress(
    'ctrl.enter',
    () => {
      sendContent();
    },
    {
      target: editorDivRef,
    }
  );

  const send_message = async () => {
    return await listen<Payload>('SendMessage', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
      editorRef.current?.getInstance().reset();
      if (conversationsSelectedRef.current.type === 'stream') {
        mutate(
          `/get_stream_message/${getLocaleStorageUtil('gid')}/${
            conversationsSelectedRef.current.body.stream
          }`
        );
      }
      if (conversationsSelectedRef.current.type === 'topic') {
        mutate(
          `/get_topic_message/${getLocaleStorageUtil('gid')}/${
            streamSelectedRef.current.stream
          }/${conversationsSelectedRef.current?.body}`
        );
      }
      // if (!!topicComboboxSelectedRef.current.new) {
      mutate(
        `/get_topic_list/${getLocaleStorageUtil('gid')}/${
          streamComboboxSelectedRef.current.stream
        }`
      );
      // }
    });
  };

  useEffect(() => {
    send_message();
  }, []);

  return (
    <div ref={editorDivRef} className={classNames('grow')}>
      <Editor
        height="100%"
        ref={editorRef}
        useCommandShortcut
        usageStatistics={false}
        hideModeSwitch
        // onChange={handleChange}
        // plugins={[colorSyntax]}
        initialEditType="markdown"
        toolbarItems={[
          [
            {
              el: createSendButton(),
              name: 'sendItem',
              tooltip: '发送',
            },
          ],
          ['heading', 'bold', 'italic', 'strike'],
          ['hr', 'quote'],
          ['ul', 'ol', 'task', 'indent', 'outdent'],
          ['table', 'image', 'link'],
          ['code', 'codeblock'],
        ]}
        placeholder={'Message #general > swimming turtles'}
        language="zh-CN"
      />
    </div>
  );
};

export default MarkdownEditor;
