import useGetPrivateChatList from '@/hooks/useGetPrivateChatList';
import { indexContext } from '@/utils/createContext';
import { Disclosure, RadioGroup } from '@headlessui/react';
import classNames from 'classnames';
import { useContext } from 'react';

const PrivateMesssagesPartBuilder = () => {
  const { conversationsSelected, setconversationsSelected, setstreamSelected } =
    useContext(indexContext);

  // const { private_chat_list } = useGetPrivateChatList('1605822843254673409');
  // useEffect(() => {
  //   console.log('private_chat_list', private_chat_list);
  // }, [private_chat_list]);

  return (
    <Disclosure>
      <Disclosure.Button
        className={classNames(
          'w-full hover:bg-[#E4E5E7] h-[42px] rounded-lg text-left px-2 '
        )}
      >
        PRIVATE MESSAGES
      </Disclosure.Button>
      <Disclosure.Panel>
        <RadioGroup
          value={conversationsSelected}
          onChange={value => {
            setconversationsSelected({
              body: value,
              type: 'private message',
            });
          }}
        >
          <div>
            {/* {private_chat_list?.map((chat: any) => (
              <RadioGroup.Option
                key={chat}
                value={chat}
                onClick={() => {
                  setstreamSelected(undefined);
                }}
                className={({ active, checked }) =>
                  `${
                    checked ? 'bg-[#DBEBF4] font-medium' : 'hover:bg-[#CADBD9]'
                  }
            cursor-pointer rounded-lg px-2 focus:outline-none  `
                }
              >
                <RadioGroup.Description
                  as="span"
                  className={classNames('flex items-center gap-x-2')}
                >
                  <div
                    className={classNames(
                      `rounded-full w-3 h-3 ${
                        true ? 'bg-green-500' : 'bg-gray-400'
                      }`
                    )}
                  ></div>
                  <span>{chat}</span>
                </RadioGroup.Description>
              </RadioGroup.Option>
            ))} */}
          </div>
        </RadioGroup>
      </Disclosure.Panel>
    </Disclosure>
  );
};

export default PrivateMesssagesPartBuilder;
