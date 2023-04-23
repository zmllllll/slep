import { indexContext } from '@/utils/createContext';
import { useClickAway } from 'ahooks';
import classNames from 'classnames';
import format from 'date-fns/format';
import dynamic from 'next/dynamic';
import { Dispatch, SetStateAction, useContext, useRef } from 'react';

const DynamicMarkdownViewer = dynamic(
  () => import('@/components/MarkdownViewer'),
  {
    ssr: false,
  }
);

const MessageBuilder = ({
  message,
  messageBodyFocusId,
  setmessageBodyFocusId,
}: {
  message: Record<string, any>[];
  messageBodyFocusId: number;
  setmessageBodyFocusId: Dispatch<SetStateAction<number>>;
}) => {
  const {
    conversationsSelected,
    streamSelected,
    editorByButtonType,
    topicComboboxSelected,
    settopicComboboxSelected,
    seteditorByButtonType,
    messageBodyFocusFlag,
  } = useContext(indexContext);

  const AwayRef = useRef<HTMLDivElement>(null);
  useClickAway(() => {
    messageBodyFocusFlag.current = false;
  }, AwayRef);

  return (
    <div
      className={classNames('flex flex-col text-xs ', {
        'opacity-50':
          message[0].topic !== topicComboboxSelected.body &&
          editorByButtonType !== undefined,
      })}
    >
      {/* 头部栏 */}
      <div
        className={classNames(
          'h-6 bg-[#e0e0e0] w-full flex items-center justify-between sticky top-0 z-50'
        )}
      >
        <div className={classNames('h-full flex items-center gap-x-1')}>
          <div className={classNames('flex h-full hover:cursor-pointer ')}>
            <span
              className={classNames(
                'font-normal px-2 bg-[#76ce90] h-full flex items-center'
              )}
            >
              {conversationsSelected.type === 'stream'
                ? conversationsSelected.body.stream
                : streamSelected.stream}
            </span>
            <div
              className={classNames(
                'w-0 h-0 border-y-[12px] border-l-[8px] border-solid border-transparent border-l-[#76ce90] '
              )}
            ></div>
          </div>
          <span className={classNames('font-bold')}>{message[0].topic}</span>
        </div>
        {/* <span className={classNames('font-semibold text-[#878787] ')}>
          今日
        </span> */}
      </div>
      {/* 内容体 */}
      {message.map(message => {
        return (
          <div
            key={message.id}
            ref={AwayRef}
            onClick={() => {
              messageBodyFocusFlag.current = true;
              setmessageBodyFocusId(message.id);
              seteditorByButtonType('default');
              settopicComboboxSelected({ body: message.topic, type: 'topic' });
            }}
            style={{
              boxShadow:
                messageBodyFocusId === message.id
                  ? 'inset 0 0 0 4px #4375bb'
                  : 'inset 4px 0px 0px 0px #76ce90',
            }}
            className={classNames('px-3 py-3 cursor-pointer flex flex-col')}
          >
            <div className={classNames('flex justify-between  items-center')}>
              <h3
                className={classNames('font-semibold', {
                  'text-red-300': message.typ === 'bot',
                })}
              >
                {message.typ === 'bot'
                  ? `机器人-->${message.username}`
                  : message.username}
              </h3>
              <span className={classNames('shrink-0 text-xs text-[#A7B2B8] ')}>
                {format(
                  new Date(JSON.parse(message.timestamp)),
                  'yyyy-MM-dd HH:mm:ss'
                )}
              </span>
            </div>
            <div className={classNames('flex')}>
              <DynamicMarkdownViewer content={message.content} />
            </div>
          </div>
        );
      })}
    </div>
  );
};

export default MessageBuilder;
