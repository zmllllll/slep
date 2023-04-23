import classNames from 'classnames';
import dynamic from 'next/dynamic';

const DynamicTopRightOption = dynamic(
  () => import('@/components/TopRightOption'),
  {
    ssr: false,
  }
);

type LayoutProps = {
  children: React.ReactNode;
};

const LoginLayout = ({ children }: LayoutProps) => {
  return (
    <div className={classNames(' h-screen flex flex-col')}>
      <div
        data-tauri-drag-region
        className={classNames(
          'h-[43px] shrink-0 bg-gradient-to-r from-[#394A6A] to-[#293451] px-[10px] py-2 flex justify-between items-center '
        )}
      >
        <div></div>
        <DynamicTopRightOption />
      </div>
      <div
        className={classNames(
          'flex grow bg-gradient-to-b from-[#394A6A] to-[#293451]'
        )}
      >
        <main className={classNames('grow h-full bg-white')}>{children}</main>
      </div>
    </div>
  );
};

export default LoginLayout;
