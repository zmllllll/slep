import { Viewer } from '@toast-ui/react-editor';
import classNames from 'classnames';
import { useEffect } from 'react';

const MarkdownViewer = ({ content }: { content: string }) => {
  useEffect(() => {
    const aList = document
      .getElementById('message_part')
      ?.querySelectorAll('a[href]');
    aList?.forEach(item => {
      if (item.getAttribute('href') !== '#') {
        item.setAttribute(
          'onclick',
          `window.__TAURI__.shell.open('${item.getAttribute('href')}')`
        );
        item.setAttribute('href', '#');
      }
    });
  }, []);
  return (
    <div className={classNames('grow')}>
      <Viewer initialValue={content} />
    </div>
  );
};

export default MarkdownViewer;
