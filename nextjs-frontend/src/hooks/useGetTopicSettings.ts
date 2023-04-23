import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetTopicSettings = (
  gid: undefined | string,
  stream: undefined | string,
  topic: undefined | string
) => {
  const { data, error, isLoading } = useSWR(
    !!gid && !!stream && !!topic
      ? `/get_topic_settings/${gid}/${stream}/${topic}`
      : null,
    () => {
      return invoke('get_topic_settings', {
        gid,
        stream,
        topic,
      });
    }
  );

  return {
    topic_settings:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' && jsonBigIntUtil(data),
    isLoading,
    error,
  };
};

export default useGetTopicSettings;
