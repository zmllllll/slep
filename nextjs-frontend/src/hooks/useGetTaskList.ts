import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import orderBy from 'lodash/orderBy';
import useSWR from 'swr';

const useGetTaskList = (gid: undefined | string) => {
  const { data, error, isLoading } = useSWR(
    !!gid ? `get_task_list/${gid}` : null,
    () => {
      return invoke('get_task_list', { gid });
    }
  );

  return {
    task_list:
      typeof data === 'undefined'
        ? undefined
        : typeof data === 'string' &&
          orderBy(
            Object.values(jsonBigIntUtil(data)),
            ['task_timestamp'],
            'desc'
          ),
    isLoading,
    error,
  };
};

export default useGetTaskList;
