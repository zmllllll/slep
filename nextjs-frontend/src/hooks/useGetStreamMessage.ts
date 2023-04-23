import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import groupBy from 'lodash/groupBy';
import useSWR from 'swr';

const useGetStreamMessage = (gid: undefined | string, stream: string) => {
  const { data, error, isLoading } = useSWR(
    !!gid && !!stream ? `/get_stream_message/${gid}/${stream}` : null,
    () => {
      return invoke('get_stream_message', {
        gid,
        stream,
      });
    }
  );

  return {
    stream_message:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' &&
          groupBy(jsonBigIntUtil(data), o => {
            return o.topic;
          }),
    isLoading,
    error,
  };
};

export default useGetStreamMessage;
