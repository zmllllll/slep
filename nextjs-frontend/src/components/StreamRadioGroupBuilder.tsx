import useGetTopicList from '@/hooks/useGetTopicList';
import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import { RadioGroup } from '@headlessui/react';
import classNames from 'classnames';
import { useContext, useEffect, useState } from 'react';
import TopicRadioGroupBuilder from './TopicRadioGroupBuilder';

const StreamRadioGroupBuilder = ({
  stream,
}: {
  stream: {
    stream: string;
    des?: any;
    rlevel: number;
    wlevel: number;
  };
}) => {
  const {
    conversationsSelected,
    setconversationsSelected,
    streamSelected,
    setstreamSelected,
  } = useContext(indexContext);

  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);
  const { topic_list } = useGetTopicList(local_gid, stream.stream);
  // console.log('topic_list', topic_list);
  // useEffect(() => {
  //   if (!!topic_list) {
  //     console.log('topic_list', topic_list);
  //   }
  // }, [topic_list]);

  return (
    <div
      onClick={() => {
        if (conversationsSelected?.body?.stream === stream.stream) {
          setconversationsSelected({});
          setstreamSelected(undefined);
        } else {
          setstreamSelected(stream);
        }
      }}
    >
      <div
        className={classNames(
          `flex flex-col rounded-lg  ${
            conversationsSelected.body?.stream === stream.stream
              ? 'bg-[#DBEBF4]'
              : ''
          }`
        )}
      >
        <div
          onClick={() => {
            setconversationsSelected({ body: stream, type: 'stream' });
          }}
          className={classNames(
            `flex items-center gap-x-1 cursor-pointer px-2 h-6 rounded-lg ${
              conversationsSelected.body?.stream === stream.stream
                ? 'bg-[#DBEBF4] font-medium'
                : 'hover:bg-[#CADBD9]'
            }`
          )}
        >
          {stream.rlevel === 3 && stream.wlevel === 3 ? (
            <span
              className={classNames('text-[#76ce90] font-extrabold text-base ')}
            >
              #&nbsp;
            </span>
          ) : (
            <svg
              className={classNames('w-4 h-4')}
              viewBox="0 0 1024 1024"
              version="1.1"
              xmlns="http://www.w3.org/2000/svg"
              p-id="13431"
              width="200"
              height="200"
            >
              <path
                d="M787.16806 952.268282 236.83194 952.268282c-30.395264 0-55.033407-24.638143-55.033407-55.033407L181.798533 429.449889c0-30.395264 24.638143-55.033407 55.033407-55.033407l82.550111 0 0-110.066815c0-106.379842 86.238107-192.617949 192.617949-192.617949s192.617949 86.238107 192.617949 192.617949l0 110.066815 82.550111 0c30.395264 0 55.033407 24.638143 55.033407 55.033407l0 467.784986C842.201467 927.630139 817.562301 952.268282 787.16806 952.268282zM484.483296 672.046113l0 115.121947 55.033407 0 0-115.121947c31.990598-11.373025 55.033407-41.605583 55.033407-77.496002 0-45.592384-36.957727-82.550111-82.550111-82.550111s-82.550111 36.957727-82.550111 82.550111C429.449889 630.440529 452.491675 660.673088 484.483296 672.046113zM622.066815 264.348644c0-60.787458-49.279357-110.066815-110.066815-110.066815s-110.066815 49.279357-110.066815 110.066815l0 110.066815 220.134653 0L622.067838 264.348644z"
                p-id="13432"
                fill="#fae589"
              ></path>
            </svg>
          )}
          <span className={classNames('text-sm')}>{stream.stream}</span>
        </div>
        {streamSelected?.stream === stream.stream && (
          <RadioGroup
            value={conversationsSelected.body}
            onChange={value => {
              setconversationsSelected({
                body: value,
                type: 'topic',
              });
            }}
          >
            <div
              onClick={e => {
                e.stopPropagation();
              }}
              className={classNames('flex flex-col')}
            >
              {topic_list?.map((topic: any) => (
                <TopicRadioGroupBuilder
                  key={topic}
                  topic={topic}
                  stream={stream.stream}
                />
              ))}
            </div>
          </RadioGroup>
        )}
      </div>
    </div>
  );
};

export default StreamRadioGroupBuilder;
