import {
  For,
  Match,
  Show,
  Switch,
  createEffect,
  createSignal,
  useContext,
} from 'solid-js';
import { register } from '@tauri-apps/api/globalShortcut';
import { isRegistered } from '@tauri-apps/api/globalShortcut';
import { appWindow } from '@tauri-apps/api/window';
import { useKeyDownEvent } from '@solid-primitives/keyboard';

import './App.scss';
import SearchResult from './SearchResult';
import { StoreContext } from './store';
import {
  hide,
  toggle_main_window,
  toggle_settings_window,
} from './invocations';
import { QueryMode } from './constants';
import Preview from './components/Preview';
import QueryResultList from './components/QueryResultList';

const loadingState = (
  <For each={[1, 2, 3, 4, 5, 6]}>
    {() => (
      <div
        class="shimmerBG"
        style={{
          width: '100%',
          height: '5rem',
        }}
      />
    )}
  </For>
);

function ActionSelector() {
  const actions = Object.keys(QueryMode).map((str) => ({
    title: str,
  }));
  return (
    <div class="action-selector">
      <For each={actions}>
        {(item, i) => (
          <div class="action active">
            <span>{item.title}</span>
          </div>
        )}
      </For>
    </div>
  );
}

function App() {
  const [
    state,
    {
      openCursor,
      openSelected,
      cursorUp,
      cursorDown,
      setCursor,
      set_search_string,
      set_search_mode,
    },
  ] = useContext(StoreContext);

  let inputRef!: HTMLInputElement;
  let ref;

  const keyboardEvent = useKeyDownEvent();

  const focus = () => {
    if (inputRef) {
      inputRef.focus();
    }
  };

  createEffect(async () => {
    const u = await appWindow.onFocusChanged(async ({ payload: focused }) => {
      // if (!focused) {
      //   await hide();
      // }
    });

    const un = await appWindow.listen('_', (e) => {
      console.log('KEYPRESS', e);
    });

    const exists = await isRegistered('CommandOrControl+K');
    // if (!exists) {
    await register('CommandOrControl+K', async () => {
      focus();
      set_search_string('');
      setCursor(0);
      await toggle_main_window();
    });
  });

  createEffect(async () => {
    const event = keyboardEvent();
    if (!event) return;

    const { key, shiftKey, ctrlKey, metaKey, altKey } = event;
    focus();

    console.log(key);
    switch (key) {
      case ',': {
        if (metaKey) {
          return toggle_settings_window();
        }
        break;
      }
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9': {
        if (metaKey || ctrlKey) {
          openSelected(key);
        }
        return;
      }
      case 'ArrowUp': {
        if (!state.search_string && state.cursor === 0) {
          // show last command/search string
          return;
        } else {
          return cursorUp();
        }
      }
      case 'ArrowDown':
        return cursorDown();
      case 'Enter': {
        return openCursor();
      }
      case 'Escape': {
        await hide();
        set_search_string('');
        return;
      }
    }
  });

  return (
    <div ref={ref} class="search-container">
      <div class="handle draggable-area" data-tauri-drag-region />
      <div class="search-input-container draggable-area" data-tauri-drag-region>
        <input
          ref={inputRef}
          onKeyDown={(event) => {
            if (event.key == 'ArrowDown' || event.key == 'ArrowUp') {
              event.preventDefault();
            }
          }}
          onFocusOut={() => inputRef && inputRef.focus()}
          autofocus={true}
          type="text"
          spellcheck={false}
          class="search-input"
          value={state.search_string}
          onInput={(e) => {
            set_search_string(e.currentTarget.value);
          }}
        />
      </div>
      <ActionSelector />
      <div class="details-container">
        <Show
          when={state.queryResult && state.queryResult.length}
          fallback={loadingState}
        >
          <QueryResultList />
          <Preview />
        </Show>
      </div>
      <ActionSelector />
      wut
    </div>
  );
}

export default App;
