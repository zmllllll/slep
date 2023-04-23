import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetGroupByUid = (uid: undefined | string) => {
  const { data, error, isLoading, mutate } = useSWR(
    !!uid ? `/get_group_by_uid/` + uid : null,
    () => {
      return invoke('get_group_by_uid', {
        uid,
      });
    }
  );

  return {
    group_list:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' && jsonBigIntUtil(data),
    isLoading,
    error,
    mutate,
  };
};

export default useGetGroupByUid;
