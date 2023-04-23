import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import { Dialog, Transition } from '@headlessui/react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import classNames from 'classnames';
import { Fragment, useContext, useEffect, useRef, useState } from 'react';
import { useSWRConfig } from 'swr';
import { ReceiptDialogStateType } from '../pages/oa';

const ReceiptDialog = ({
  ReceiptDialogState,
  closeDialog,
}: {
  ReceiptDialogState: ReceiptDialogStateType;
  closeDialog: () => void;
}) => {
  const { mutate } = useSWRConfig();
  const { streamSelected } = useContext(indexContext);
  const streamSelectedRef = useRef(streamSelected);
  streamSelectedRef.current = streamSelected;
  const receiptFinish = () => {
    invoke('add_task_receipt', {
      hashkey: ReceiptDialogState.task_info?.hashkey,
      taskId: ReceiptDialogState.task_info?.id,
      executor: getLocaleStorageUtil('uid'),
      status: (document.getElementById('status') as HTMLSelectElement).value,
      des: (document.getElementById('receipt_des') as HTMLTextAreaElement)
        .value,
    });
  };

  const SendMessage = async () => {
    return await listen<any>('SendMessage', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload);
      mutate(`get_task_list/${getLocaleStorageUtil('gid')}`);
      mutate(
        `/get_topic_list/${getLocaleStorageUtil('gid')}/${
          streamSelectedRef.current.stream
        }`
      );
      closeDialog();
    });
  };
  useEffect(() => {
    SendMessage();
  }, []);

  const [warnDialogVisible, setwarnDialogVisible] = useState(false);

  return (
    <Transition appear show={ReceiptDialogState.ReceiptDialogVisible} as="div">
      <Dialog
        as="div"
        className="relative z-10"
        onClose={() => {
          if (
            !!(document.getElementById('status') as HTMLSelectElement).value ||
            !!(document.getElementById('receipt_des') as HTMLTextAreaElement)
              .value
          ) {
            setwarnDialogVisible(true);
          } else {
            closeDialog();
          }
        }}
      >
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

        <div className="fixed inset-0  overflow-y-auto">
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
              <Dialog.Panel className="sm:w-4/5 xl:w-1/2 transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all">
                <Dialog.Title
                  as="h3"
                  className="text-lg font-semibold leading-6 text-gray-900"
                >
                  添加任务回执
                </Dialog.Title>
                <form
                  className="mt-4 space-y-6 text-gray-500"
                  autoComplete="off"
                  onSubmit={e => {
                    e.preventDefault();
                    receiptFinish();
                  }}
                  id="receipt_form"
                >
                  <div className="grid grid-cols-1 gap-4 rounded-md shadow-sm">
                    <div>
                      <label htmlFor="status">回执状态</label>
                      <select
                        id="status"
                        required
                        className="relative block sm:w-48 xl:w-64 appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                      >
                        <option className={classNames('hidden')}></option>
                        <option value="created">创建</option>
                        <option value="pending">进行中</option>
                        <option value="confirmed">完成</option>
                        <option value="blocked">阻塞</option>
                        <option value="failed">拒绝</option>
                      </select>
                    </div>
                    <div>
                      <label htmlFor="receipt_des">回执描述</label>
                      <textarea
                        id="receipt_des"
                        name="receipt_des"
                        rows={10}
                        className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                        placeholder="回执描述"
                      />
                    </div>
                  </div>
                  <div>
                    <button
                      type="submit"
                      className="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                    >
                      确定
                    </button>
                  </div>
                </form>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
      {warnDialogVisible && (
        <Transition appear show={warnDialogVisible} as={Fragment}>
          <Dialog
            as="div"
            className="relative z-10"
            onClose={() => {
              setwarnDialogVisible(false);
            }}
          >
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
                      className="text-2xl font-semibold leading-6 text-gray-900"
                    >
                      你的内容未保存是否退出？
                    </Dialog.Title>
                    <div className={classNames('mt-8 text-right space-x-4')}>
                      <button
                        onClick={() => {
                          closeDialog();
                        }}
                        className="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                      >
                        确定
                      </button>
                      <button
                        onClick={() => {
                          setwarnDialogVisible(false);
                        }}
                        className="inline-flex justify-center rounded-md border border-transparent bg-gray-100 px-4 py-2 text-sm font-medium  hover:bg-gray-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                      >
                        取消
                      </button>
                    </div>
                  </Dialog.Panel>
                </Transition.Child>
              </div>
            </div>
          </Dialog>
        </Transition>
      )}
    </Transition>
  );
};

export default ReceiptDialog;
