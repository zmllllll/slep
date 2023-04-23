import { getLocaleStorageUtil, setLocaleStorageUtil } from '@/utils/helper';
import { Dialog, Transition } from '@headlessui/react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { useRouter } from 'next/router';
import { Fragment, useEffect } from 'react';

const AddGroupDialog = ({
  AddGroupDialogVisible,
  closeDialog,
}: {
  AddGroupDialogVisible: boolean;
  closeDialog: () => void;
}) => {
  const router = useRouter();
  const addGroupFinish = () => {
    invoke('add_member', {
      gid: (document.getElementById('gid') as HTMLInputElement).value,
      uid: getLocaleStorageUtil('uid'),
      level: 1,
    });
  };
  const add_member = async () => {
    return await listen<any>('AddMember', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
      setLocaleStorageUtil(
        'gid',
        (document.getElementById('gid') as HTMLInputElement).value
      );
      router.push('/');
    });
  };
  useEffect(() => {
    add_member();
  }, []);

  return (
    <Transition appear show={AddGroupDialogVisible} as={Fragment}>
      <Dialog as="div" className="relative z-10" onClose={closeDialog}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-black bg-opacity-25" />
        </Transition.Child>

        <div className="fixed inset-0 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4 text-center">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 scale-95"
              enterTo="opacity-100 scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 scale-100"
              leaveTo="opacity-0 scale-95"
            >
              <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all">
                <Dialog.Title
                  as="h3"
                  className="text-lg font-semibold leading-6 text-gray-900"
                >
                  加入机构
                </Dialog.Title>
                <form
                  className="mt-4 space-y-6 text-gray-500"
                  autoComplete="off"
                  onSubmit={e => {
                    e.preventDefault();
                    addGroupFinish();
                  }}
                  id="receipt_form"
                >
                  <div className="grid grid-cols-1 gap-4 rounded-md shadow-sm">
                    <div>
                      <label htmlFor="gid">组id</label>
                      <input
                        id="gid"
                        name="gid"
                        type="text"
                        autoComplete="off"
                        required
                        className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                        placeholder="请输入组id"
                      />
                    </div>
                  </div>
                  <div>
                    <button
                      type="submit"
                      className="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                    >
                      加入
                    </button>
                  </div>
                </form>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  );
};

export default AddGroupDialog;
