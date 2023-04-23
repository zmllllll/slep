import useGetStreamMessage from '@/hooks/useGetStreamMessage';
import useGetTopicMessage from '@/hooks/useGetTopicMessage';
import styles from '@/styles/myStyles.module.scss';
import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import classNames from 'classnames';
import sortBy from 'lodash/sortBy';
import { ReactNode, useContext, useEffect, useState } from 'react';
import MessageBuilder from './MessageBuilder';

const MessageViewer = () => {
  const { conversationsSelected, streamSelected } = useContext(indexContext);
  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);

  const { stream_message } = useGetStreamMessage(
    local_gid,
    conversationsSelected.type === 'stream'
      ? conversationsSelected.body.stream
      : undefined
  );
  const { topic_message } = useGetTopicMessage(
    local_gid,
    conversationsSelected.type === 'topic' ? streamSelected.stream : undefined,
    conversationsSelected?.body
  );

  // useEffect(() => {
  //   if (!!stream_message) {
  //     console.log('stream_message', stream_message);
  //   }
  // }, [stream_message]);
  // useEffect(() => {
  //   if (!!topic_message) {
  //     console.log('topic_message', topic_message);
  //   }
  // }, [topic_message]);

  const [messageBodyFocusId, setmessageBodyFocusId] = useState(0);

  const handerMessage = (message: any) => {
    const messageNode: ReactNode[] = [];
    for (const key in message) {
      messageNode.push(
        <MessageBuilder
          key={key}
          message={sortBy(message[key], ['timestamp'], ['asc'])}
          messageBodyFocusId={messageBodyFocusId}
          setmessageBodyFocusId={setmessageBodyFocusId}
        />
      );
    }
    return messageNode;
  };

  return (
    <div
      className={classNames(
        'w-full grow flex  gap-y-3 h-0 overflow-y-scroll pb-72 ',
        styles.noScrollbar,
        { 'flex-col-reverse': conversationsSelected.type !== 'stream' },
        { 'flex-col': conversationsSelected.type === 'stream' }
      )}
      id="message_part"
    >
      {conversationsSelected.type !== 'stream' && (
        <div className={classNames('grow w-full')}></div>
      )}
      {conversationsSelected?.type === 'stream'
        ? handerMessage(stream_message)
        : handerMessage(topic_message)}
    </div>
  );
};

export default MessageViewer;
