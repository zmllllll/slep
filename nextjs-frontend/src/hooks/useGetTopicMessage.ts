import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import groupBy from 'lodash/groupBy';
import useSWR from 'swr';

const useGetTopicMessage = (
  gid: undefined | string,
  stream: string,
  topic: string
) => {
  const { data, error, isLoading } = useSWR(
    !!stream && !!gid ? `/get_topic_message/${gid}/${stream}/${topic}` : null,
    () => {
      return invoke('get_topic_message', {
        gid,
        stream,
        topic,
      });
    }
  );

  return {
    topic_message:
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

export default useGetTopicMessage;
