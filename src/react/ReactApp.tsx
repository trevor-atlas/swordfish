import { writeText } from '@tauri-apps/api/clipboard';
import { open } from '@tauri-apps/api/shell';
import { appWindow } from '@tauri-apps/api/window';
import { useCallback, useRef } from 'react';
import '../App.scss';
import {
  BROWSER_HISTORY,
  CHAT,
  QUERY_MODES,
  SCRIPTS,
  SEARCH,
} from '../constants';
import { ActionSelector } from '../feature/action-selector/ActionSelector';
import { Chat } from '../feature/chat/Chat';
import { useInputHandler } from '../hooks/useInputHandler';
import { toggle_settings_window } from '../invocations';
import { QueryInput } from './QueryInput';
import ResultList from './ResultList';
import { getSelectedResult, openResult, useStore } from './reactStore';
import Details from '../feature/details/Details';

const Loading = () => (
  <div
    style={{
      width: '100%',
      height: '5rem',
    }}
  />
);

await appWindow.onFocusChanged(async ({ payload: focused }) => {
  if (!focused) {
    // await hide();
  }
});

function useInput(inputRef: React.RefObject<HTMLInputElement>) {
  return useInputHandler(
    useCallback(async (event: KeyboardEvent) => {
      if (inputRef.current) {
        setTimeout(() => inputRef.current?.focus?.(), 0);
      }
      const {
        cursorUp,
        cursorDown,
        resetAndHide,
        nextSearchMode,
        prevSearchMode,
      } = useStore.getState();

      if (!event) return;
      focus();

      const handleNumericalSelection = async (
        key: number,
        event: KeyboardEvent,
      ) => {
        const { ctrlKey, metaKey } = event;
        if (!(metaKey || ctrlKey)) {
          return;
        }
        const value = getSelectedResult(key)?.subheading;
        if (value) {
          await open(value);
          await resetAndHide();
        }
      };
      const { key, shiftKey, ctrlKey, metaKey, altKey, location } = event;
      const hasModifier = shiftKey || ctrlKey || metaKey || altKey;

      if (hasModifier) {
        if (location === 1) console.log(`left `);
        if (location === 2) console.log(`right `);
      }
      console.log('keypress', {
        key,
        shiftKey,
        ctrlKey,
        metaKey,
        altKey,
        location,
      });

      switch (key) {
        case 'Tab': {
          event.preventDefault();
          event.stopPropagation();
          if (shiftKey) {
            return prevSearchMode();
          }
          nextSearchMode();
          break;
        }
        case ',': {
          if (metaKey) {
            return toggle_settings_window();
          }
          break;
        }
        case '1':
          return handleNumericalSelection(1, event);
        case '2':
          return handleNumericalSelection(2, event);
        case '3':
          return handleNumericalSelection(3, event);
        case '4':
          return handleNumericalSelection(4, event);
        case '5':
          return handleNumericalSelection(5, event);
        case '6':
          return handleNumericalSelection(6, event);
        case '7':
          return handleNumericalSelection(7, event);
        case '8':
          return handleNumericalSelection(8, event);
        case '9': {
          return handleNumericalSelection(9, event);
        }
        case 'c':
        case 'C':
          if ((metaKey || ctrlKey) && shiftKey) {
            const value = getSelectedResult()?.subheading;
            if (value) {
              await writeText(value);
            }
          }
          break;
        case 'ArrowUp': {
          // if (!state.search_string && state.cursor === 0) {
          // show last command/search string
          // return;
          // } else {
          return cursorUp();
          // }
        }
        case 'ArrowDown':
          return cursorDown();
        case 'Enter': {
          const value = getSelectedResult();
          await openResult(value);
          break;
        }
        case 'Escape': {
          await resetAndHide();
          return;
        }
      }
    }, []),
  );
}

function App() {
  const inputRef = useRef<HTMLInputElement>(null);
  useInput(inputRef);

  return (
    <div className="search-container">
      <div className="handle draggable-area" data-tauri-drag-region />
      <QueryInput inputRef={inputRef} />
      <ActionSelector />
      <Results />
      {/* <pre>{JSON.stringify(useStore.getState(), null, 2)}</pre> */}
    </div>
  );
}

function Results() {
  const { mode } = useStore();
  switch (QUERY_MODES[mode]) {
    case SEARCH:
    case BROWSER_HISTORY:
    case SCRIPTS:
      return (
        <div className="detail-container">
          <ResultList />
          <Details />
        </div>
      );
    case CHAT:
      return <Chat />;
    default:
      throw new Error('Invalid mode') as never;
  }
}

function Footer() {
  return (
    <div className="flex flex-row border-t border-ui-border bg-ui-bg max-h-6.5 min-h-6.5 h-6.5 items-center justify-center overflow-hidden px-4 pt-px">
      :O
    </div>
  );
}

export default App;
