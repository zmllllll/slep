import useGetStreamList from '@/hooks/useGetStreamList';
import useGetTopicList from '@/hooks/useGetTopicList';
import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import {
  ChevronDownIcon,
  ChevronRightIcon,
  ChevronUpIcon,
  XMarkIcon,
} from '@heroicons/react/20/solid';
import { useClickAway, useFullscreen } from 'ahooks';
import classNames from 'classnames';
import dynamic from 'next/dynamic';
import { Resizable } from 're-resizable';
import { useContext, useEffect, useRef, useState } from 'react';
import ComboboxBuilder from './ComboboxBuilder';

const DynamicMarkdownEditor = dynamic(
  () => import('@/components/MarkdownEditor'),
  {
    ssr: false,
  }
);

const BottomControl = () => {
  const {
    conversationsSelected,
    streamSelected,
    streamComboboxSelected,
    editorByButtonType,
    seteditorByButtonType,
    messageBodyFocusFlag,
    setCreateStreamVisible,
  } = useContext(indexContext);

  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);

  const { stream_list } = useGetStreamList(local_gid);
  const { topic_list } = useGetTopicList(
    local_gid,
    streamSelected?.stream ?? stream_list?.[0]?.stream
  );

  const AwayRef = useRef<HTMLDivElement>(null);
  useClickAway(() => {
    if (!messageBodyFocusFlag.current) {
      seteditorByButtonType(undefined);
    }
  }, AwayRef);

  const streamPartRef = useRef<any>(null);
  const topicPartRef = useRef<any>(null);
  useEffect(() => {
    switch (editorByButtonType) {
      case 'topic':
        topicPartRef.current?.focus();
        break;

      default:
        break;
    }
  }, [editorByButtonType]);

  const fullScreenRef = useRef(null);
  const [isFullscreen, { exitFullscreen, toggleFullscreen }] =
    useFullscreen(fullScreenRef);

  const [streamWarn, setstreamWarn] = useState(false);

  return (
    <div ref={AwayRef}>
      <div
        className={classNames(
          `h-10  border border-gray-300 ${
            editorByButtonType === undefined ? 'flex' : 'hidden'
          }  items-center gap-x-1 px-1 text-sm text-[#333333] font-normal`
        )}
      >
        {streamSelected !== undefined ? (
          <button
            className={classNames(
              'flex-grow rounded-md border border-gray-300 px-[10px] py-[3px] text text-left focus:outline-none truncate '
            )}
            onClick={() => {
              seteditorByButtonType('default');
            }}
          >
            Message #{streamSelected?.stream} {'>'}{' '}
            {conversationsSelected.type === 'stream'
              ? topic_list[0]
              : conversationsSelected.body}
          </button>
        ) : (
          <button
            className={classNames(
              'flex-grow rounded-md border border-gray-300 px-[10px] py-[3px] text text-left focus:outline-none '
            )}
            onClick={() => {
              seteditorByButtonType('default');
            }}
          >
            To {conversationsSelected.body}
          </button>
        )}
        <button
          className={classNames(
            'focus:outline-none border border-gray-300 rounded-md px-[10px] py-[3px] shrink-0 '
          )}
          onClick={() => {
            seteditorByButtonType('topic');
          }}
        >
          New topic
        </button>
        <button
          className={classNames(
            'focus:outline-none border border-gray-300 rounded-md px-[10px] py-[3px] shrink-0 '
          )}
          onClick={() => {
            seteditorByButtonType('private message');
          }}
        >
          New private message
        </button>
      </div>
      <div
        className={classNames(
          `${editorByButtonType !== undefined ? 'block' : 'hidden'}`
        )}
      >
        <Resizable
          defaultSize={{
            width: '100%',
            height: 200,
          }}
          enable={{ top: true }}
          minHeight={200}
          maxHeight={350}
        >
          <div
            ref={fullScreenRef}
            className={classNames(
              'h-full px-3 pt-3 pb-1 border border-gray-300 flex flex-col gap-y-2 bg-white'
            )}
          >
            {streamWarn && (
              <div
                className={classNames(
                  'w-full flex items-center justify-between text-sm h-10 text-[#842923] bg-[#eedddc] border border-[#C49592] rounded-md px-3 '
                )}
              >
                <div>
                  The stream&nbsp;
                  <b className={classNames('text-[#842923] ')}>
                    {streamComboboxSelected.stream}
                  </b>
                  &nbsp;does not exist. Manage your subscriptions&nbsp;
                  <a
                    className={classNames(
                      'text-[#08c] hover:cursor-pointer hover:text-[#0052cc] '
                    )}
                    onClick={() => {
                      setCreateStreamVisible(true);
                    }}
                  >
                    on your Streams page
                  </a>
                  .
                </div>
                <XMarkIcon
                  onClick={e => {
                    setstreamWarn(false);
                    e.stopPropagation();
                  }}
                  className={classNames('w-5 h-5')}
                />
              </div>
            )}
            <div
              className={classNames(
                'h-7 shrink-0 flex items-center justify-between'
              )}
            >
              <div className={classNames('flex items-center')}>
                <div className={classNames('w-[132px]')}>
                  <ComboboxBuilder
                    ComboboxValue={stream_list}
                    ref={streamPartRef}
                    ComboboxPlaceholder="Stream"
                  />
                </div>
                <ChevronRightIcon className={classNames('w-5 h-5')} />
                <div className={classNames('w-[400px]')}>
                  <ComboboxBuilder
                    ComboboxValue={topic_list}
                    ref={topicPartRef}
                    ComboboxPlaceholder="Topic"
                  />
                </div>
              </div>
              <div className={classNames('flex items-center')}>
                {isFullscreen ? (
                  <ChevronDownIcon
                    onClick={toggleFullscreen}
                    className={classNames('w-5 h-5')}
                  />
                ) : (
                  <ChevronUpIcon
                    onClick={toggleFullscreen}
                    className={classNames('w-5 h-5')}
                  />
                )}
                <XMarkIcon
                  onClick={() => {
                    exitFullscreen();
                    seteditorByButtonType(undefined);
                  }}
                  className={classNames('w-5 h-5')}
                />
              </div>
            </div>
            <DynamicMarkdownEditor setstreamWarn={setstreamWarn} />
          </div>
        </Resizable>
      </div>
    </div>
  );
};

export default BottomControl;
