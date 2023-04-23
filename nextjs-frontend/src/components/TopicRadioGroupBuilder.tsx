import useGetTaskList from '@/hooks/useGetTaskList';
import useGetTopicSettings from '@/hooks/useGetTopicSettings';
import { getLocaleStorageUtil } from '@/utils/helper';
import { RadioGroup } from '@headlessui/react';
import classNames from 'classnames';
import orderBy from 'lodash/orderBy';
import { useEffect, useState } from 'react';

const TopicRadioGroupBuilder = ({
  topic,
  stream,
}: {
  topic: string;
  stream: string;
}) => {
  const [local_gid, setlocal_gid] = useState<string | undefined>(undefined);
  useEffect(() => {
    setlocal_gid(getLocaleStorageUtil('gid'));
  }, []);
  const { task_list } = useGetTaskList(local_gid);
  const { topic_settings } = useGetTopicSettings(
    local_gid,
    stream,
    !!task_list ? topic : undefined
  );
  const [topic_task, settopic_task] = useState<Record<string, any>>([]);
  useEffect(() => {
    // console.log('task_list', task_list);
    // console.log('topic_settings', topic_settings);
    if (!!topic_settings) {
      const tem = (task_list as Array<Record<string, any>>).filter(item => {
        return item.id === topic_settings.associate_task_id;
      });
      settopic_task(tem);
    }
  }, [topic_settings?.associate_task_id, JSON.stringify(task_list)]);

  return (
    <RadioGroup.Option
      key={topic}
      value={topic}
      className={({ active, checked }) =>
        classNames(
          `cursor-pointer rounded-lg px-9 py-2 my-1 focus:outline-none`,
          {
            // 最后回执状态为confirmed，blocked，failed，置于末尾
            'order-last':
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status === 'confirmed' ||
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status === 'blocked' ||
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status === 'failed',
            // 最后回执状态为confirmed
            [` text-green-900 ${
              checked
                ? 'bg-green-500 font-medium'
                : 'bg-green-300 hover:bg-green-500 '
            }`]:
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status === 'confirmed',
            // 最后回执状态为blocked，failed
            [` text-red-900 ${
              checked
                ? 'bg-red-500 font-medium'
                : 'bg-red-300 hover:bg-red-500 '
            }`]:
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status === 'blocked' ||
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status === 'failed',
            // 最后回执状态不为failed,blocked,confirmed时（即正常的状态）
            [checked ? 'bg-[#DBEBF4] font-medium' : 'hover:bg-[#CADBD9]']:
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status !== 'failed' &&
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status !== 'blocked' &&
              orderBy(
                topic_task?.[0]?.receipts,
                ['receipt_timestamp'],
                ['desc']
              )?.[0]?.status !== 'confirmed',
          }
        )
      }
    >
      <RadioGroup.Description
        as="span"
        className={classNames('flex items-center gap-x-2')}
      >
        <span className={classNames(`text-sm `)}>{topic}</span>
      </RadioGroup.Description>
    </RadioGroup.Option>
  );
};

export default TopicRadioGroupBuilder;
