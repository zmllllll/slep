import { indexContext } from '@/utils/createContext';
import { Combobox, Transition } from '@headlessui/react';
import { CheckIcon } from '@heroicons/react/20/solid';
import { useSetState } from 'ahooks';
import classNames from 'classnames';
import {
  forwardRef,
  Fragment,
  useContext,
  useEffect,
  useImperativeHandle,
  useRef,
} from 'react';

const ComboboxBuilder = forwardRef(
  (
    props: {
      ComboboxValue: any[];
      ComboboxPlaceholder: string;
    },
    pRef: any
  ) => {
    const {
      conversationsSelected,
      streamSelected,
      topicComboboxSelected,
      settopicComboboxSelected,
      streamComboboxSelected,
      setstreamComboboxSelected,
      editorByButtonType,
    } = useContext(indexContext);

    const { ComboboxValue = [], ComboboxPlaceholder } = props;

    useEffect(() => {
      if (editorByButtonType) {
        if (ComboboxPlaceholder === 'Stream') {
          // 开始构造stream combobox数据源
          setstreamComboboxSelected(
            // 有选中stream的情况,用选中的stream
            // 没有选中用第一个stream
            streamSelected ? { ...streamSelected } : { ...ComboboxValue[0] }
          );
        }
        if (ComboboxPlaceholder === 'Topic') {
          // 开始构造stream combobox数据源
          if (
            !conversationsSelected.type ||
            conversationsSelected.type === 'stream'
          ) {
            if (!topicComboboxSelected.type) {
              // 如果topicComboboxSelected没值 才给他赋值
              settopicComboboxSelected({
                body: ComboboxValue[0],
                type: 'topic',
              });
            }
          }
          if (conversationsSelected.type === 'topic') {
            settopicComboboxSelected({ ...conversationsSelected });
          }
        }
      }
    }, [editorByButtonType]);

    const ComboboxRef = useRef<any>(null);

    useImperativeHandle(pRef, () => ({
      focus: () => ComboboxRef.current?.focus(),
    }));

    const [queryStream, setqueryStream] = useSetState<Record<string, any>>({
      query_field: '',
      filtered_values: [],
    });

    useEffect(() => {
      if (ComboboxPlaceholder === 'Stream') {
        if (queryStream.query_field === '') {
          setqueryStream({
            filtered_values: [],
          });
        } else {
          setqueryStream({
            filtered_values: ComboboxValue.filter(value =>
              value.stream
                .toLowerCase()
                .replace(/\s+/g, '')
                .includes(
                  queryStream.query_field.toLowerCase().replace(/\s+/g, '')
                )
            ),
          });
        }
      } else {
        if (queryStream.query_field === '') {
          setqueryStream({
            filtered_values: [],
          });
        } else {
          setqueryStream({
            filtered_values: ComboboxValue.filter(value =>
              value
                .toLowerCase()
                .replace(/\s+/g, '')
                .includes(
                  queryStream.query_field.toLowerCase().replace(/\s+/g, '')
                )
            ),
          });
        }
      }
    }, [queryStream.query_field]);

    return (
      <Combobox
        by={ComboboxPlaceholder === 'Stream' ? 'stream' : 'body'}
        value={
          ComboboxPlaceholder === 'Stream'
            ? streamComboboxSelected
            : topicComboboxSelected
        }
        onChange={value => {
          if (ComboboxPlaceholder === 'Stream') {
            setstreamComboboxSelected({ ...value });
          } else {
            settopicComboboxSelected({ ...value });
          }
        }}
      >
        <div className={classNames('relative')}>
          <div>
            <Combobox.Input
              autoComplete="off"
              className={classNames(
                'w-full px-[6px] h-[25.5px] rounded focus:outline-none text-sm text-[#5A5555] border border-[#5A5555] '
              )}
              displayValue={(stream: Record<string, any>) =>
                ComboboxPlaceholder === 'Stream' ? stream?.stream : stream?.body
              }
              onChange={e => {
                setqueryStream({
                  query_field: e.target.value,
                });
              }}
              placeholder={ComboboxPlaceholder}
              ref={ComboboxRef}
            />
          </div>
          <Transition
            as={Fragment}
            leave="transition ease-in duration-100"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
            afterLeave={() =>
              setqueryStream({
                query_field: '',
              })
            }
          >
            <Combobox.Options
              className={classNames(
                'z-50 absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-sm shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none'
              )}
            >
              {queryStream.query_field.length > 0 &&
                queryStream.filtered_values.length === 0 && (
                  <Combobox.Option
                    value={
                      ComboboxPlaceholder === 'Stream'
                        ? {
                            stream: queryStream.query_field,
                            new: true,
                          }
                        : {
                            body: queryStream.query_field,
                            type: 'topic',
                            new: true,
                          }
                    }
                    className={({ active }) =>
                      `relative cursor-default select-none py-1 pl-10 pr-4 ${
                        active ? 'bg-teal-600 text-white' : 'text-gray-900'
                      }`
                    }
                  >
                    Create {queryStream.query_field}
                  </Combobox.Option>
                )}
              {queryStream.filtered_values.length !== 0 &&
                queryStream.filtered_values.map(
                  (stream: Record<string, any>) => {
                    return (
                      <Combobox.Option
                        key={
                          ComboboxPlaceholder === 'Stream'
                            ? stream.stream
                            : stream
                        }
                        value={
                          ComboboxPlaceholder === 'Stream'
                            ? stream
                            : {
                                body: stream,
                                type: 'topic',
                              }
                        }
                        className={({ active }) =>
                          `relative cursor-default select-none py-1 pl-10 pr-4 ${
                            active ? 'bg-teal-600 text-white' : 'text-gray-900'
                          }`
                        }
                      >
                        {({ selected, active }) => (
                          <>
                            <span
                              className={`block truncate ${
                                selected ? 'font-medium' : 'font-normal'
                              }`}
                            >
                              {ComboboxPlaceholder === 'Stream'
                                ? stream.stream
                                : stream}
                            </span>
                            {selected ? (
                              <span
                                className={`absolute inset-y-0 left-0 flex items-center pl-3 ${
                                  active ? 'text-white' : 'text-teal-600'
                                }`}
                              >
                                <CheckIcon
                                  className="h-5 w-5"
                                  aria-hidden="true"
                                />
                              </span>
                            ) : null}
                          </>
                        )}
                      </Combobox.Option>
                    );
                  }
                )}
            </Combobox.Options>
          </Transition>
        </div>
      </Combobox>
    );
  }
);

export default ComboboxBuilder;
