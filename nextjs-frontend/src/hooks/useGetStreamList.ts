import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetStreamList = (gid: undefined | string) => {
  const { data, error, isLoading } = useSWR(
    !!gid ? `/get_stream_list/${gid}` : null,
    () => {
      return invoke('get_stream_list', {
        gid,
      });
    }
  );

  return {
    stream_list:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' && jsonBigIntUtil(data),
    isLoading,
    error,
  };
};

export default useGetStreamList;
