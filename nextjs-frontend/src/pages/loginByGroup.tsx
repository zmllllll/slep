import LoginLayout from '@/components/LoginLayout';
import useGetGroupByUid from '@/hooks/useGetGroupByUid';
import {
  getLocaleStorageUtil,
  jsonBigIntUtil,
  setLocaleStorageUtil,
} from '@/utils/helper';
import { listen } from '@tauri-apps/api/event';
import classNames from 'classnames';
import { useRouter } from 'next/router';
import { ReactElement, useEffect, useState } from 'react';
import AddGroupDialog from '../components/AddGroupDialog';

const loginByGroup = () => {
  const router = useRouter();
  const create_group = async () => {
    return await listen<any>('CreateGroup', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
      setLocaleStorageUtil('gid', jsonBigIntUtil(event.payload.data).Upsert.id);
      router.push('/');
    });
  };
  useEffect(() => {
    create_group();
  }, []);

  const [AddGroupDialogVisible, setAddGroupDialogVisible] = useState(false);

  const [local_uid, setlocal_uid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_uid(getLocaleStorageUtil('uid'));
  }, []);
  const { group_list } = useGetGroupByUid(local_uid);

  return (
    <div className="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <div>
          <h2 className=" text-center text-3xl font-bold tracking-tight text-gray-900">
            请选择以哪个机构登录？
          </h2>
        </div>
        <div className={classNames('space-y-2')}>
          {group_list?.map((item: any) => {
            return (
              <div
                key={item.gid}
                className={classNames(
                  'text-center px-2 py-3 rounded-lg shadow-lg hover:cursor-pointer'
                )}
                onClick={() => {
                  setLocaleStorageUtil('gid', item.gid);
                  router.push('/');
                }}
              >
                {item.des}
              </div>
            );
          })}
        </div>
        <div
          className={classNames(
            'text-center px-2 py-3 rounded-lg shadow-lg hover:cursor-pointer bg-indigo-600 text-white'
          )}
          onClick={() => {
            router.push('/createGroup');
          }}
        >
          创建结构
        </div>
      </div>
      {AddGroupDialogVisible && (
        <AddGroupDialog
          AddGroupDialogVisible={AddGroupDialogVisible}
          closeDialog={() => {
            setAddGroupDialogVisible(false);
          }}
        />
      )}
    </div>
  );
};

loginByGroup.getLayout = function getLayout(page: ReactElement) {
  return <LoginLayout>{page}</LoginLayout>;
};

export default loginByGroup;
