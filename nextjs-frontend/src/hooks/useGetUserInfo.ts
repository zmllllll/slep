import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetUserInfo = (uid: undefined | string) => {
  const { data, error, isLoading } = useSWR(
    !!uid ? `/get_user_info/${uid}` : null,
    () => {
      return invoke('get_user_info', {
        uid,
      });
    }
  );

  return {
    user_info:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' && jsonBigIntUtil(data),
    isLoading,
    error,
  };
};

export default useGetUserInfo;
