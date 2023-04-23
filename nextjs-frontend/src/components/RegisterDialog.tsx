import { Dialog, Transition } from '@headlessui/react';
import { Fragment, useEffect } from 'react';

const RegisterDialog = ({
  RegisterDialogVisible,
  closeDialog,
}: {
  RegisterDialogVisible: boolean;
  closeDialog: () => void;
}) => {
  const registerFinish = () => {
    fetch('https://1to2to3.cn/slep-register/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json; charset=utf-8',
      },
      body: JSON.stringify({
        id: Number(
          (document.getElementById('user_id') as HTMLInputElement).value
        ),
        name: (document.getElementById('username') as HTMLInputElement).value,
      }),
    }).then(res => {
      if (res.status === 200) {
        closeDialog();
      }
    });
  };

  return (
    <Transition appear show={RegisterDialogVisible} as={Fragment}>
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
                  注册
                </Dialog.Title>
                <form
                  className="mt-4 space-y-6 text-gray-500"
                  autoComplete="off"
                  onSubmit={e => {
                    e.preventDefault();
                    registerFinish();
                  }}
                >
                  <div className="grid grid-cols-1 gap-4 rounded-md shadow-sm">
                    <div>
                      <label htmlFor="user_id">用户id</label>
                      <input
                        id="user_id"
                        name="user_id"
                        type="text"
                        autoComplete="off"
                        required
                        className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                        placeholder="请输入用户id"
                      />
                    </div>
                    <div>
                      <label htmlFor="username">昵称</label>
                      <input
                        id="username"
                        name="username"
                        type="text"
                        autoComplete="off"
                        required
                        className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                        placeholder="昵称"
                      />
                    </div>
                  </div>
                  <div>
                    <button
                      type="submit"
                      className="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                    >
                      注册
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

export default RegisterDialog;
