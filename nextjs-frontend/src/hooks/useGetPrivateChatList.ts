import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetPrivateChatList = (uid: string) => {
  const { data, error, isLoading } = useSWR(
    `/get_private_chat_list/${uid}`,
    () => {
      return invoke('get_private_chat_list', {
        uid,
      });
    }
  );

  return {
    private_chat_list:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' && jsonBigIntUtil(data),
    isLoading,
    error,
  };
};

export default useGetPrivateChatList;
