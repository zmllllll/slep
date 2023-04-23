import useGetStreamList from '@/hooks/useGetStreamList';
import styles from '@/styles/myStyles.module.scss';
import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import classNames from 'classnames';
import { useContext, useEffect, useState } from 'react';
import CreateStreamDialog from './CreateStreamDialog';
import StreamRadioGroupBuilder from './StreamRadioGroupBuilder';

const StreamsPartBuilder = () => {
  const { CreateStreamVisible, setCreateStreamVisible } =
    useContext(indexContext);
  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);

  const { stream_list } = useGetStreamList(local_gid);

  // useEffect(() => {
  //   if (!!stream_list) {
  //     console.log('stream_list', stream_list);
  //   }
  // }, [stream_list]);

  return (
    <div className={classNames('flex flex-col grow h-full ')}>
      <button
        className={classNames(
          'w-full hover:bg-[#E4E5E7] h-[42px] rounded-lg text-left px-2 focus:outline-none '
        )}
      >
        STREAMS
      </button>
      <div
        className={classNames(
          'grow h-0 overflow-y-scroll ',
          styles.noScrollbar
        )}
      >
        {stream_list?.map((stream: any) => {
          return (
            <StreamRadioGroupBuilder key={stream.stream} stream={stream} />
          );
        })}
        <a
          className={classNames(
            'text-[#08c] px-2 text-sm hover:cursor-pointer'
          )}
          onClick={() => {
            setCreateStreamVisible(true);
          }}
        >
          Create a stream
        </a>
      </div>
      <CreateStreamDialog
        CreateStreamVisible={CreateStreamVisible}
        closeDialog={() => {
          setCreateStreamVisible(false);
        }}
      />
    </div>
  );
};

export default StreamsPartBuilder;
