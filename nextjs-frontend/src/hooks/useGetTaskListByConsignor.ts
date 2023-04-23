import { jsonBigIntUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import orderBy from 'lodash/orderBy';
import useSWR from 'swr';

const useGetTaskListByConsignor = (uid: undefined | string) => {
  const { data, error, isLoading } = useSWR(
    !!uid ? `/get_task_list_by_consignor/${uid}` : null,
    () => {
      return invoke('get_task_list_by_consignor', {
        consignor: uid,
      });
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

export default useGetTaskListByConsignor;
