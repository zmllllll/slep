import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import useSWR from 'swr';

const useGetGroupMemberList = (gid: undefined | string) => {
  const { data, error, isLoading } = useSWR(
    !!gid ? `/get_group_member_list/${gid}` : null,
    () => {
      return invoke('get_group_member_list', {
        gid,
      });
    }
  );

  return {
    group_member_list:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' && jsonBigIntUtil(data),
    isLoading,
    error,
  };
};

export default useGetGroupMemberList;
