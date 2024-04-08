import { Show } from 'solid-js';
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

  let ref!: HTMLLIElement;

  const handleMouseEvent = (event: MouseEvent) => {
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
      <div class="result-content">
        <div class="flex items-center">
          <Show when={iconPath}>
            <img
              class="result-icon"
              src={convertFileSrc(iconPath)}
              alt="icon"
            />
          </Show>
          <div class="flex flex-col">
            <span class="result-heading">{heading || <NoTitle />}</span>
            <span class="result-subtext">{subtext}</span>
            <KeyboardShortcuts />
          </div>
        </div>
      </div>
    </li>
  );
}

const NoTitle = () => <span>� no title :( �</span>;

const KeyboardShortcuts = () => (
  <div class="keyboard-shortcuts">
    <KBDCopy />
  </div>
);

const KBDCopy = () => (
  <span class="keyboard-shortcut">
    copy <kbd>⌘</kbd>
    <kbd>⇧</kbd>
    <kbd>C</kbd>
  </span>
);
