import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetTopicList = (
  gid: undefined | string,
  stream: string | undefined
) => {
  const { data, error, isLoading } = useSWR(
    !!stream && !!gid ? `/get_topic_list/${gid}/${stream}` : null,
    () => {
      return invoke('get_topic_list', {
        gid,
        stream,
      });
    }
  );

  return {
    topic_list:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' &&
          jsonBigIntUtil(data).map((item: Record<string, any>) => {
            return item.topic;
          }),
    isLoading,
    error,
  };
};

export default useGetTopicList;
