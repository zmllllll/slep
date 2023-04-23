import Layout from '@/components/Layout';
import useGetGroupByUid from '@/hooks/useGetGroupByUid';
import useGetGroupMemberList from '@/hooks/useGetGroupMemberList';
import styles from '@/styles/myStyles.module.scss';
import { getLocaleStorageUtil } from '@/utils/helper';
import classNames from 'classnames';
import { ReactElement, useEffect, useState } from 'react';
import { NextPageWithLayout } from '../_app';

const index: NextPageWithLayout = () => {
  const [uid, setuid] = useState<undefined | string>(undefined);
  useEffect(() => {
    setuid(getLocaleStorageUtil('uid'));
  }, []);

  const { group_list } = useGetGroupByUid(uid);
  const [select_group, setselect_group] = useState<Record<string, any>>();

  useEffect(() => {
    if (select_group === undefined && !!group_list) {
      setselect_group(group_list[0]);
    }
  }, [JSON.stringify(group_list)]);

  const { group_member_list } = useGetGroupMemberList(select_group?.gid);
  // console.log('group_member_list', group_member_list);

  return (
    <div className={classNames('flex h-full')}>
      <div
        className={classNames(
          'w-60 h-full shrink-0 bg-[#F5F6F7] rounded-tl-xl px-[10px] py-5 overflow-y-scroll',
          styles.noScrollbar
        )}
      >
        <div className={classNames('text-xl pl-4 mb-8')}>用户组</div>
        <div className={classNames('flex flex-col space-y-2')}>
          {group_list?.map((item: any) => {
            return (
              <div
                key={item.gid}
                className={classNames(
                  'w-full h-10 rounded-xl bg-red-300 flex justify-center items-center text-white text-base font-medium hover:cursor-pointer'
                )}
                onClick={() => {
                  setselect_group(item);
                }}
              >
                {item.name}
              </div>
            );
          })}
        </div>
      </div>
      <div className={classNames('grow h-full flex flex-col px-[10px] py-5')}>
        <div className={classNames('text-xl pl-4 mb-8')}>
          {select_group?.name}--成员列表
        </div>
        <div
          className={classNames(
            'flex flex-col grow h-0 overflow-y-scroll space-y-2',
            styles.noScrollbar
          )}
        >
          {group_member_list?.map((item: any) => {
            return (
              <div
                key={item.uid}
                className={classNames(
                  'w-[200px] h-10 shrink-0 rounded-lg border border-gray-200 text-base font-medium flex justify-center items-center'
                )}
              >
                {item.name}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};

index.getLayout = function getLayout(page: ReactElement) {
  return <Layout>{page}</Layout>;
};

export default index;
