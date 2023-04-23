import LoginLayout from '@/components/LoginLayout';
import {
  getLocaleStorageUtil,
  jsonBigIntUtil,
  setLocaleStorageUtil,
} from '@/utils/helper';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import classNames from 'classnames';
import { useRouter } from 'next/router';
import { ReactElement, useEffect, useState } from 'react';
import AddGroupDialog from '../components/AddGroupDialog';

const createGroup = () => {
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

  return (
    <div className="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <div>
          <h2 className=" text-center text-3xl font-bold tracking-tight text-gray-900">
            请选择加入机构或者创建机构？
          </h2>
        </div>
        <form
          className="mt-8 space-y-6"
          autoComplete="off"
          onSubmit={e => {
            e.preventDefault();
            invoke('create_group', {
              pid: null,
              uid: getLocaleStorageUtil('uid'),
              groupName: (
                document.getElementById('group_name') as HTMLInputElement
              ).value,
              des: (document.getElementById('des') as HTMLTextAreaElement)
                .value,
            });
          }}
        >
          <input type="hidden" name="remember" defaultValue="true" />
          <div>
            <label htmlFor="group_name">组名称</label>
            <input
              id="group_name"
              name="group_name"
              type="text"
              autoComplete="off"
              required
              className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              placeholder="组名称"
            />
          </div>
          <div>
            <label htmlFor="des">组描述</label>
            <textarea
              id="des"
              name="des"
              required
              rows={3}
              className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              placeholder="组描述"
            />
          </div>

          <div className={classNames('grid grid-cols-2 gap-4')}>
            <button
              type="submit"
              className="group relative flex w-full justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            >
              创建
            </button>
            <button
              type="button"
              className="group relative flex w-full justify-center rounded-md border border-indigo-600 bg-white py-2 px-4 text-sm font-medium text-black  focus:outline-none "
              onClick={() => {
                setAddGroupDialogVisible(true);
              }}
            >
              加入
            </button>
          </div>
        </form>
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

createGroup.getLayout = function getLayout(page: ReactElement) {
  return <LoginLayout>{page}</LoginLayout>;
};

export default createGroup;
