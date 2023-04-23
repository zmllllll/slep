import LoginLayout from '@/components/LoginLayout';
import { jsonBigIntUtil, setLocaleStorageUtil } from '@/utils/helper';
import { LockClosedIcon } from '@heroicons/react/20/solid';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import classNames from 'classnames';
import { useRouter } from 'next/router';
import { ReactElement, useEffect, useState } from 'react';
import { NextPageWithLayout } from '../_app';
import RegisterDialog from '../../components/RegisterDialog';

const index: NextPageWithLayout = () => {
  const router = useRouter();

  const initialize = async () => {
    return await listen<any>('Login', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload.data);

      setLocaleStorageUtil('uid', jsonBigIntUtil(event.payload.data).Upsert.id);
      invoke('get_group_by_uid', {
        uid: jsonBigIntUtil(event.payload.data).Upsert.id,
      }).then((res: any) => {
        if (JSON.parse(res).length === 0) {
          // 去创建组或者加入机构
          router.push('/createGroup');
        } else {
          router.push('/loginByGroup');
        }
      });
    });
  };

  useEffect(() => {
    initialize();
  }, []);

  // useEffect(() => {
  //   invoke('greet', {
  //     id: 1627569304857923584,
  //   }).then((res: any) => {
  //     console.log('res', res);
  //   });
  // }, []);

  const [RegisterDialogVisible, setRegisterDialogVisible] = useState(false);

  return (
    <div className="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <div>
          {/* <img
            className="mx-auto h-12 w-auto"
            src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
            alt="Your Company"
          /> */}
          <h2 className="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
            Sign in to your account
          </h2>
        </div>
        <form
          className="mt-8 space-y-6"
          autoComplete="off"
          onSubmit={e => {
            e.preventDefault();
            invoke('login', {
              uid: (document.getElementById('account') as HTMLInputElement)
                .value,
            });
          }}
        >
          <input type="hidden" name="remember" defaultValue="true" />
          <div className="-space-y-px rounded-md shadow-sm">
            <div>
              <label htmlFor="account" className="sr-only">
                Account number
              </label>
              <input
                id="account"
                name="account"
                type="text"
                autoComplete="off"
                required
                className="relative block w-full appearance-none rounded-none rounded-t-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                placeholder="Account"
              />
            </div>
            <div>
              <label htmlFor="password" className="sr-only">
                Password
              </label>
              <input
                id="password"
                name="password"
                type="password"
                autoComplete="current-password"
                className="relative block w-full appearance-none rounded-none rounded-b-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                placeholder="Password"
              />
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <input
                id="remember-me"
                name="remember-me"
                type="checkbox"
                className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
              />
              <label
                htmlFor="remember-me"
                className="ml-2 block text-sm text-gray-900"
              >
                Remember me
              </label>
            </div>

            <div className="text-sm">
              <a
                href="#"
                className="font-medium text-indigo-600 hover:text-indigo-500"
              >
                Forgot your password?
              </a>
            </div>
          </div>

          <div className={classNames('grid grid-cols-2 gap-4')}>
            <button
              type="submit"
              className="group relative flex w-full justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            >
              <span className="absolute inset-y-0 left-0 flex items-center pl-3">
                <LockClosedIcon
                  className="h-5 w-5 text-indigo-500 group-hover:text-indigo-400"
                  aria-hidden="true"
                />
              </span>
              Sign in
            </button>
            <button
              type="button"
              className="group relative flex w-full justify-center rounded-md border border-indigo-600 bg-white py-2 px-4 text-sm font-medium text-black  focus:outline-none "
              onClick={() => {
                setRegisterDialogVisible(true);
              }}
            >
              Sign up
            </button>
          </div>
        </form>
      </div>
      {RegisterDialogVisible && (
        <RegisterDialog
          RegisterDialogVisible={RegisterDialogVisible}
          closeDialog={() => {
            setRegisterDialogVisible(false);
          }}
        />
      )}
    </div>
  );
};

index.getLayout = function getLayout(page: ReactElement) {
  return <LoginLayout>{page}</LoginLayout>;
};

export default index;
