import { listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import classNames from 'classnames';
import { useEffect, useState } from 'react';

const TopRightOption = () => {
  const [isMaximized, setisMaximized] = useState(false);
  useEffect(() => {
    listen('tauri://resize', async () => {
      setisMaximized(await appWindow.isMaximized());
    });
  }, []);

  return (
    <div className={classNames('flex')}>
      <div
        onClick={() => {
          appWindow.minimize();
        }}
        className={classNames(
          'w-[30px] h-6 group hover:bg-[#5A657D] flex justify-center items-center '
        )}
      >
        <svg
          className="w-5 h-5 "
          viewBox="0 0 1024 1024"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          p-id="3217"
          width="200"
          height="200"
        >
          <path
            d="M797.291117 486.21473 224.18848 486.21473c-14.078647 0-25.469068 11.342326-25.469068 25.472138 0 14.028505 11.390421 25.471115 25.469068 25.471115l573.101613 0c14.07967 0 25.470091-11.441587 25.470091-25.471115C822.760185 497.557056 811.370787 486.21473 797.291117 486.21473z"
            p-id="3218"
            className={classNames('fill-[#B8BCC8] group-hover:fill-white')}
          ></path>
        </svg>
      </div>
      <div
        onClick={() => {
          appWindow.toggleMaximize();
        }}
        className={classNames(
          'w-[30px] h-6 group hover:bg-[#5A657D] flex justify-center items-center '
        )}
      >
        <>
          {isMaximized ? (
            <svg
              className="w-3 h-3"
              viewBox="0 0 1024 1024"
              version="1.1"
              xmlns="http://www.w3.org/2000/svg"
              p-id="3099"
              width="200"
              height="200"
            >
              <path
                d="M959.72 0H294.216a63.96 63.96 0 0 0-63.96 63.96v127.92H64.28A63.96 63.96 0 0 0 0.32 255.84V959.4a63.96 63.96 0 0 0 63.96 63.96h703.56a63.96 63.96 0 0 0 63.96-63.96V792.465h127.92a63.96 63.96 0 0 0 63.96-63.96V63.96A63.96 63.96 0 0 0 959.72 0zM767.84 728.505V959.4H64.28V255.84h703.56z m189.322 0H831.8V255.84a63.96 63.96 0 0 0-63.96-63.96H294.216V63.96H959.72z"
                p-id="3100"
                className={classNames('fill-[#B8BCC8] group-hover:fill-white')}
              ></path>
            </svg>
          ) : (
            <svg
              className="w-3 h-3"
              viewBox="0 0 1024 1024"
              version="1.1"
              xmlns="http://www.w3.org/2000/svg"
              p-id="3350"
              width="200"
              height="200"
            >
              <path
                d="M926.45937303 97.54062697v828.2973677H97.54062697V97.54062697h828.91874606m4.97102697-77.6722963h-838.8608c-39.7682157 0-72.07989097 32.31167525-72.07989097 72.07989096v839.48217837c0 39.7682157 32.31167525 72.07989097 72.07989097 72.07989097h839.48217837c39.7682157 0 72.07989097-32.31167525 72.07989096-72.07989097v-838.8608c0-40.38959408-32.31167525-72.70126933-72.70126933-72.70126933 0.62137837 0 0 0 0 0z"
                p-id="3351"
                className={classNames('fill-[#B8BCC8] group-hover:fill-white')}
              ></path>
            </svg>
          )}
        </>
      </div>
      <div
        onClick={() => {
          appWindow.hide();
        }}
        className={classNames(
          'w-[30px] h-6 group hover:bg-red-400 flex justify-center items-center '
        )}
      >
        <svg
          className="w-3 h-3"
          viewBox="0 0 1024 1024"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          p-id="4277"
          width="200"
          height="200"
        >
          <path
            d="M548.992 503.744L885.44 167.328a31.968 31.968 0 1 0-45.248-45.248L503.744 458.496 167.328 122.08a31.968 31.968 0 1 0-45.248 45.248l336.416 336.416L122.08 840.16a31.968 31.968 0 1 0 45.248 45.248l336.416-336.416L840.16 885.44a31.968 31.968 0 1 0 45.248-45.248L548.992 503.744z"
            p-id="4278"
            className={classNames('fill-[#B8BCC8] group-hover:fill-white')}
          ></path>
        </svg>
      </div>
    </div>
  );
};

export default TopRightOption;
