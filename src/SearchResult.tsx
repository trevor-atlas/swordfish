import { MouseEventHandler, useRef } from 'react';
import { useStore } from './store';
import { Nullable } from './types';
import { convertFileSrc } from '@tauri-apps/api/tauri';

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
  const [state, { setCursor }] = useStore();

  const ref = useRef<HTMLLIElement>(null);

  const handleMouseEvent = (event: MouseEventHandler<HTMLLIElement>) => {
    if (
      event.screenX !== lastMousePos.x ||
      (event.screenY !== lastMousePos.y && state.cursor !== index)
    ) {
      setCursor(index);
    }
    lastMousePos.x = event.screenX;
    lastMousePos.y = event.screenY;
  };

  return (
    <li
      ref={ref}
      class={`${state.cursor === index ? 'active' : ''} query-result query-result-${index}`}
      onMouseEnter={handleMouseEvent}
      onMouseMove={handleMouseEvent}
    >
      <div className="result-content">
        <div className="flex items-center">
          <Show when={iconPath}>
            <img
              className="result-icon"
              src={convertFileSrc(iconPath)}
              alt="icon"
            />
          </Show>
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
