import type { NextPage } from 'next';
import type { AppProps } from 'next/app';
import { ReactElement, ReactNode, useEffect } from 'react';

import '@/styles/globals.css';
import { getLocaleStorageUtil } from '@/utils/helper';
import { invoke } from '@tauri-apps/api/tauri';
import { useRouter } from 'next/router';

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
  getLayout?: (page: ReactElement) => ReactNode;
};

type AppPropsWithLayout = AppProps & {
  Component: NextPageWithLayout;
};

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppPropsWithLayout) {
  const router = useRouter();
  console.log('_app');
  useEffect(() => {
    if (!!getLocaleStorageUtil('uid') && !!getLocaleStorageUtil('gid')) {
      invoke('login', {
        uid: BigInt(getLocaleStorageUtil('uid')).toString(),
      });
    } else {
      router.push('/login');
    }
  }, []);

  const getLayout = Component.getLayout ?? (page => page);

  return <>{getLayout(<Component {...pageProps} />)}</>;
}
