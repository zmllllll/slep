/**
 * 设置localstorage的值
 * @param name LocaleStorage名称
 * @param value 值
 */
export const setLocaleStorageUtil = (name: string, value: any) => {
  localStorage.setItem(
    `slep-${process.env.NODE_ENV}-${name}`,
    JSON.stringify(value)
  );
};

/**
 * 获取localstorage的值
 * @param name LocaleStorage名称
 * @returns LocaleStorage值
 */
export const getLocaleStorageUtil = (name: string) => {
  return JSON.parse(
    localStorage.getItem(`slep-${process.env.NODE_ENV}-${name}`) || '""'
  );
};

export const removeLocaleStorageUtil = (name: string) => {
  localStorage.removeItem(`slep-${process.env.NODE_ENV}-${name}`);
};

export const jsonBigIntUtil = (string_json: string) => {
  return JSON.parse(
    string_json.replace(/:s*([+-]?[0-9]+)s*(,?)/g, ': "$1" $2')
  );
};
