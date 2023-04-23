import Layout from '@/components/Layout';
import useGetStreamList from '@/hooks/useGetStreamList';
import useGetTaskList from '@/hooks/useGetTaskList';
import useGetTopicList from '@/hooks/useGetTopicList';
import styles from '@/styles/myStyles.module.scss';
import { getLocaleStorageUtil } from '@/utils/helper';
import { Disclosure, RadioGroup } from '@headlessui/react';
import { ChevronUpIcon, XMarkIcon } from '@heroicons/react/20/solid';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import classNames from 'classnames';
import format from 'date-fns/format';
import getTime from 'date-fns/getTime';
import orderBy from 'lodash/orderBy';
import { ReactElement, useEffect, useState } from 'react';
import { useSWRConfig } from 'swr';
import ReceiptDialog from '../../components/ReceiptDialog';

export type ReceiptDialogStateType = {
  ReceiptDialogVisible: boolean;
  task_info?: Record<string, any>;
};

const index = () => {
  const { mutate } = useSWRConfig();
  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);
  const [local_uid, setlocal_uid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_uid(getLocaleStorageUtil('uid'));
  }, []);

  const { stream_list } = useGetStreamList(local_gid);

  const [selectStream, setselectStream] = useState<undefined | string>(
    undefined
  );
  const { topic_list } = useGetTopicList(local_gid, selectStream);

  const [taskType, settaskType] = useState<'group' | 'private'>('group');
  const [topicAlreadyWarnVisible, settopicAlreadyWarnVisible] = useState(false);

  const handerFinish = () => {
    invoke('assign_task', {
      gid: getLocaleStorageUtil('gid'),
      stream: (document.getElementById('stream') as HTMLSelectElement).value,
      topic: (document.getElementById('task_name') as HTMLSelectElement).value,
      name: (document.getElementById('task_name') as HTMLSelectElement).value,
      des: (document.getElementById('des') as HTMLTextAreaElement).value,
      typ: taskType,
      consignor: getLocaleStorageUtil('uid'),
      deadline: getTime(
        new Date(
          (
            document.getElementById('deadline') as HTMLInputElement
          ).value.replace(/-/g, '/')
        )
      ).toString(),
    }).catch(res => {
      settopicAlreadyWarnVisible(true);
    });
  };

  const AssignTask = async () => {
    return await listen<any>('AssignTask', event => {
      console.log('event: ', event.event);
      console.log('payload: ', event.payload.data);
      (document.getElementById('task_form') as HTMLFormElement).reset();
      mutate(`get_task_list/${getLocaleStorageUtil('gid')}`);
      !!topicAlreadyWarnVisible && settopicAlreadyWarnVisible(false);
    });
  };
  useEffect(() => {
    AssignTask();
  }, []);

  const { task_list } = useGetTaskList(local_gid);
  console.debug('🚀 ~ file: index.tsx:80 ~ index ~ task_list:', task_list);

  const task_listBuilder = () => {
    if (!!task_list) {
      return task_list.map((item: any) => {
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

  const [ReceiptDialogState, setReceiptDialogState] =
    useState<ReceiptDialogStateType>({
      ReceiptDialogVisible: false,
    });

  return (
    <div className={classNames('grid grid-cols-2 w-full h-full')}>
      <div className={classNames('h-full w-full px-6 flex flex-col pb-6')}>
        <h1
          className={classNames('text-4xl font-semibold text-center shrink-0')}
        >
          Task List
        </h1>
        <div
          className={classNames(
            'w-full grow h-0 overflow-y-scroll mt-6 flex flex-col ',
            styles.noScrollbar
          )}
        >
          {task_listBuilder()}
        </div>
      </div>
      <div className={classNames('h-full w-full px-6')}>
        <h1 className={classNames('text-4xl font-semibold text-center')}>
          New Task
        </h1>
        <form
          className="mt-8 space-y-6"
          autoComplete="off"
          onSubmit={e => {
            e.preventDefault();
            handerFinish();
          }}
          id="task_form"
        >
          <div className="grid grid-cols-1 gap-4 rounded-md shadow-sm">
            <div>
              <RadioGroup value={taskType} onChange={settaskType}>
                {/* <RadioGroup.Label>任务类型</RadioGroup.Label> */}
                <div className={classNames('flex space-x-2')}>
                  <RadioGroup.Option value="group">
                    {({ checked }) => (
                      <span
                        className={classNames(
                          `px-3 py-2 rounded-md cursor-pointer ${
                            checked
                              ? 'text-white bg-[#1FCDEE]'
                              : 'border border-gray-300'
                          }`
                        )}
                      >
                        组任务
                      </span>
                    )}
                  </RadioGroup.Option>
                  <RadioGroup.Option value="private" disabled>
                    {({ checked }) => (
                      <span
                        className={classNames(
                          `px-3 py-2  rounded-md cursor-pointer ${
                            checked
                              ? 'text-white bg-[#1FCDEE]'
                              : ' border border-gray-300'
                          }`
                        )}
                      >
                        个人任务
                      </span>
                    )}
                  </RadioGroup.Option>
                </div>
              </RadioGroup>
            </div>
            {taskType === 'group' && (
              <div>
                <label htmlFor="stream">Stream</label>
                <select
                  id="stream"
                  required
                  className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                  onChange={() => {
                    setselectStream(
                      (document.getElementById('stream') as HTMLSelectElement)
                        .value
                    );
                  }}
                >
                  <option className={classNames('hidden')}></option>
                  {stream_list?.map((item: Record<string, any>) => {
                    return <option key={item.stream}>{item.stream}</option>;
                  })}
                </select>
              </div>
            )}
            {/* <div>
              <label htmlFor="topic">Topic</label>
              <select
                id="topic"
                required
                className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              >
                <option className={classNames('hidden')}></option>
                {topic_list?.map((item: string) => {
                  return <option key={item}>{item}</option>;
                })}
              </select>
              <input
                id="topic"
                name="topic"
                type="text"
                autoComplete="off"
                required
                className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                placeholder="topic"
              />
            </div> */}
            {topicAlreadyWarnVisible && (
              <div
                className={classNames(
                  'w-full h-[38px] rounded-md bg-[#E74C3C] px-3 py-2 text-white flex justify-between items-center '
                )}
              >
                <span>该topic已有任务</span>
                <XMarkIcon
                  onClick={e => {
                    settopicAlreadyWarnVisible(false);
                    e.stopPropagation();
                  }}
                  className={classNames('w-5 h-5 cursor-pointer')}
                />
              </div>
            )}
            <div>
              <label htmlFor="task_name">任务名称</label>
              <input
                id="task_name"
                name="task_name"
                type="text"
                autoComplete="off"
                required
                className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                placeholder="任务名称"
              />
            </div>
            <div>
              <label htmlFor="des">任务描述</label>
              <textarea
                id="des"
                name="des"
                rows={3}
                className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                placeholder="任务描述"
              />
            </div>
            <div>
              <label htmlFor="deadline">截止日期</label>
              <input
                id="deadline"
                name="deadline"
                type="date"
                required
                defaultValue={format(new Date(), 'yyyy-MM-dd')}
                className="relative block w-full appearance-none  rounded-md border border-gray-300 px-3 py-2 text-gray-900 placeholder-gray-500 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              />
            </div>
          </div>

          <div>
            <button
              type="submit"
              className="group relative flex w-full justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            >
              发布任务
            </button>
          </div>
        </form>
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

index.getLayout = function getLayout(page: ReactElement) {
  return <Layout>{page}</Layout>;
};

export default index;
