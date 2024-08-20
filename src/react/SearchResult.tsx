import { getSelectedResult, openResult, useStore } from './reactStore';
import { Nullable } from '../types';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { MouseEventHandler, useCallback, useRef } from 'react';

interface ResultProps {
  iconPath: Nullable<string>;
  heading: string;
  subtext: string;
  index: number;
}

const lastMousePos = { x: 0, y: 0 };

export default function SearchResult({
  heading,
  subtext,
  iconPath,
  index,
}: ResultProps) {
  const { cursor, setCursor } = useStore();

  const ref = useRef<HTMLLIElement>(null);

  const handleMouseEvent: MouseEventHandler<HTMLLIElement> = useCallback(
    (event) => {
      if (
        event.screenX !== lastMousePos.x ||
        (event.screenY !== lastMousePos.y && cursor !== index)
      ) {
        setCursor(index);
      }
      lastMousePos.x = event.screenX;
      lastMousePos.y = event.screenY;
    },
    [],
  );

  const handleClick = useCallback(async () => {
    const value = getSelectedResult();
    console.log('open result', value);
    await openResult(value);
  }, []);

  return (
    <li
      ref={ref}
      className={`${cursor === index ? 'active' : ''} query-result query-result-${index}`}
      onMouseEnter={handleMouseEvent}
      onMouseMove={handleMouseEvent}
      onClick={handleClick}
    >
      <div className="result-content">
        <div className="flex items-center">
          {iconPath && (
            <div className="result-icon-container">
              <img
                className="result-icon"
                src={convertFileSrc(iconPath)}
                alt="icon"
              />
            </div>
          )}
          <div className="flex flex-col">
            <span className="result-heading">{heading || <NoTitle />}</span>
            <span className="result-subtext">{subtext}</span>
            <KeyboardShortcuts />
          </div>
        </div>
      </div>
    </li>
  );
}

const NoTitle = () => <span>� no title :( �</span>;

const KeyboardShortcuts = () => (
  <div className="keyboard-shortcuts">
    <KBDCopy />
  </div>
);

const KBDCopy = () => (
  <span className="keyboard-shortcut">
    copy <kbd>⌘</kbd>
    <kbd>⇧</kbd>
    <kbd>C</kbd>
  </span>
);
