import ReceiptDialog from '@/components/ReceiptDialog';
import useGetTaskList from '@/hooks/useGetTaskList';
import useGetTopicSettings from '@/hooks/useGetTopicSettings';
import { ReceiptDialogStateType } from '@/pages/oa';
import styles from '@/styles/myStyles.module.scss';
import { indexContext } from '@/utils/createContext';
import { getLocaleStorageUtil } from '@/utils/helper';
import { Disclosure } from '@headlessui/react';
import { ChevronUpIcon } from '@heroicons/react/20/solid';
import classNames from 'classnames';
import format from 'date-fns/format';
import orderBy from 'lodash/orderBy';
import { useContext, useEffect, useState } from 'react';

const TopicInfo = () => {
  const { conversationsSelected, streamSelected } = useContext(indexContext);
  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);
  const { task_list } = useGetTaskList(local_gid);

  const { topic_settings } = useGetTopicSettings(
    local_gid,
    streamSelected?.stream,
    !!task_list && conversationsSelected.type === 'topic'
      ? conversationsSelected?.body
      : undefined
  );


  const [topic_task, settopic_task] = useState<Record<string, any>>([]);

  useEffect(() => {
    if (!!topic_settings) {
      const tem = (task_list as Array<Record<string, any>>).filter(item => {
        return item.id === topic_settings.associate_task_id;
      });
      settopic_task(tem);
    }else{
      settopic_task([])
    }
  }, [topic_settings?.associate_task_id, JSON.stringify(task_list)]);

  const [ReceiptDialogState, setReceiptDialogState] =
    useState<ReceiptDialogStateType>({
      ReceiptDialogVisible: false,
    });

  const task_listBuilder = () => {
    if (!!topic_task) {
      return topic_task.map((item: any) => {
        return (
          <Disclosure
            className={classNames(
              `${
                orderBy(item.receipts, ['receipt_timestamp'], ['desc'])?.[0]
                  ?.status === 'confirmed' ||
                orderBy(item.receipts, ['receipt_timestamp'], ['desc'])?.[0]
                  ?.status === 'blocked' ||
                orderBy(item.receipts, ['receipt_timestamp'], ['desc'])?.[0]
                  ?.status === 'failed'
                  ? 'order-last'
                  : ''
              } my-1`
            )}
            key={item.id}
            as="div"
            defaultOpen
          >
            {({ open }) => (
              <>
                <Disclosure.Button
                  className={classNames(
                    `flex w-full justify-between rounded-lg ${
                      orderBy(
                        item.receipts,
                        ['receipt_timestamp'],
                        ['desc']
                      )?.[0]?.status === 'confirmed'
                        ? 'bg-green-100'
                        : orderBy(
                            item.receipts,
                            ['receipt_timestamp'],
                            ['desc']
                          )?.[0]?.status === 'blocked' ||
                          orderBy(
                            item.receipts,
                            ['receipt_timestamp'],
                            ['desc']
                          )?.[0]?.status === 'failed'
                        ? 'bg-red-100'
                        : 'bg-purple-100'
                    } px-4 py-2 text-left text-sm font-medium ${
                      orderBy(
                        item.receipts,
                        ['receipt_timestamp'],
                        ['desc']
                      )?.[0]?.status === 'confirmed'
                        ? 'text-green-900 focus-visible:ring-green-500 hover:bg-green-200'
                        : orderBy(
                            item.receipts,
                            ['receipt_timestamp'],
                            ['desc']
                          )?.[0]?.status === 'blocked' ||
                          orderBy(
                            item.receipts,
                            ['receipt_timestamp'],
                            ['desc']
                          )?.[0]?.status === 'failed'
                        ? 'text-red-900 focus-visible:ring-red-500 hover:bg-red-200'
                        : 'text-purple-900 focus-visible:ring-purple-500 hover:bg-purple-200'
                    }   focus:outline-none focus-visible:ring  focus-visible:ring-opacity-75`
                  )}
                >
                  <span>{item.name}</span>
                  <ChevronUpIcon
                    className={`${
                      open ? 'rotate-180 transform' : ''
                    } h-5 w-5 text-purple-500`}
                  />
                </Disclosure.Button>
                <Disclosure.Panel className=" pt-4 pb-2 text-sm text-gray-500">
                  <dl>
                    <div
                      className={classNames(
                        ' px-4 grid grid-cols-3 gap-4  odd:bg-gray-100 even:bg-white'
                      )}
                    >
                      <dt className="text-sm font-medium text-gray-500 py-3">
                        任务类型
                      </dt>
                      <dd className=" text-sm text-gray-900 col-span-2 mt-0 flex justify-between items-center">
                        <span className={classNames('py-3')}>
                          {item.typ === 'group' ? '组任务' : '个人任务'}
                        </span>
                        <button
                          className={classNames(
                            'cursor-pointer rounded-lg px-3 py-2 bg-indigo-600 hover:bg-indigo-700 text-white'
                          )}
                          onClick={() => {
                            setReceiptDialogState({
                              ReceiptDialogVisible: true,
                              task_info: item,
                            });
                          }}
                        >
                          回执
                        </button>
                      </dd>
                    </div>
                    <div
                      className={classNames(
                        ' px-4 grid grid-cols-3 gap-4 py-3 odd:bg-gray-100 even:bg-white'
                      )}
                    >
                      <dt className="text-sm font-medium text-gray-500">
                        委托人
                      </dt>
                      <dd className=" text-sm text-gray-900 col-span-2 mt-0">
                        {item.consignor_name}
                      </dd>
                    </div>
                    <div
                      className={classNames(
                        ' px-4 grid grid-cols-3 gap-4 py-3 odd:bg-gray-100 even:bg-white'
                      )}
                    >
                      <dt className="text-sm font-medium text-gray-500">
                        截止时间
                      </dt>
                      <dd className=" text-sm text-gray-900 col-span-2 mt-0">
                        {format(new Date(Number(item.deadline)), 'yyyy-MM-dd')}
                      </dd>
                    </div>
                    <div
                      className={classNames(
                        ' px-4 grid grid-cols-3 gap-4 py-3 odd:bg-gray-100 even:bg-white'
                      )}
                    >
                      <dt className="text-sm font-medium text-gray-500">
                        任务描述
                      </dt>
                      <dd className=" text-sm text-gray-900 col-span-2 mt-0 break-words">
                        {item.task_des}
                      </dd>
                    </div>
                    <div
                      className={classNames(
                        ' px-4 grid grid-cols-3 gap-4 py-3 odd:bg-gray-100 even:bg-white'
                      )}
                    >
                      <dt className="text-sm font-medium text-gray-500">
                        回执列表
                      </dt>
                      <dd className=" text-sm text-gray-900 col-span-2 mt-0">
                        <ol
                          className={classNames(
                            'list-decimal marker:text-sky-400'
                          )}
                        >
                          {orderBy(
                            item.receipts,
                            ['receipt_timestamp'],
                            ['asc']
                          ).map(item => {
                            return (
                              <li key={item.receipt_timestamp}>
                                {item.executor_name}-{item.status}：
                                {item.receipt_des}
                              </li>
                            );
                          })}
                        </ol>
                      </dd>
                    </div>
                  </dl>
                </Disclosure.Panel>
              </>
            )}
          </Disclosure>
        );
      });
    }
  };

  return (
    <div className={classNames('w-full h-full flex flex-col')}>
      <h1 className={classNames('text-xl font-bold ')}>topic info</h1>
      <div
        className={classNames(
          'w-full grow h-0 overflow-y-scroll mt-6 space-y-2',
          styles.noScrollbar
        )}
      >
        {task_listBuilder()}
      </div>
      {ReceiptDialogState.ReceiptDialogVisible && (
        <ReceiptDialog
          ReceiptDialogState={ReceiptDialogState}
          closeDialog={() => {
            setReceiptDialogState({
              ReceiptDialogVisible: false,
            });
          }}
        />
      )}
    </div>
  );
};

export default TopicInfo;
