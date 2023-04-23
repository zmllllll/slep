import { Payload } from '@/pages';
import { getLocaleStorageUtil } from '@/utils/helper';
import { Dialog, Transition } from '@headlessui/react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { Fragment, useEffect } from 'react';
import { useSWRConfig } from 'swr';

const CreateStreamDialog = ({
  CreateStreamVisible,
  closeDialog,
}: {
  CreateStreamVisible: boolean;
  closeDialog: () => void;
}) => {
  const { mutate } = useSWRConfig();
  const CreateStreamFinish = () => {
    invoke('update_stream_settings', {
      gid: getLocaleStorageUtil('gid'),
      stream: (document.getElementById('stream_name') as HTMLInputElement)
        .value,
      des: (document.getElementById('stream_des') as HTMLInputElement).value,
      rlevel: 1,
      wlevel: 1,
    });
  };
  const update_stream_settings = async () => {
    return await listen<Payload>('UpdateStreamSettings', event => {
      mutate(`/get_stream_list/${getLocaleStorageUtil('gid')}`);
      closeDialog();
    });
  };
  useEffect(() => {
    update_stream_settings();
  }, []);

  return (
    <Transition appear show={CreateStreamVisible} as={Fragment}>
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
                  创建Stream
                </Dialog.Title>
                <form
                  className="mt-4 space-y-6 text-gray-500"
                  autoComplete="off"
                  onSubmit={e => {
                    e.preventDefault();
                    CreateStreamFinish();
                  }}
                  id="receipt_form"
                >
                  <div className="grid grid-cols-1 gap-4 rounded-md shadow-sm">
                    <div>
                      <label htmlFor="stream_name">Stream name</label>
                      <input
                        id="stream_name"
                        name="stream_name"
                        type="text"
                        autoComplete="off"
                        required
                        className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                        placeholder="stream_name"
                      />
                    </div>
                    <div>
                      <label htmlFor="stream_des">Stream description</label>
                      <input
                        id="stream_des"
                        name="stream_des"
                        type="text"
                        autoComplete="off"
                        required
                        className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                        placeholder="Stream description"
                      />
                    </div>
                  </div>
                  <div>
                    <button
                      type="submit"
                      className="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                    >
                      创建
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

export default CreateStreamDialog;
