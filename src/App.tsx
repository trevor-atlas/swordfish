import { For, Match, Show, Switch, createEffect, useContext } from 'solid-js';
import { register } from '@tauri-apps/api/globalShortcut';
import { isRegistered } from '@tauri-apps/api/globalShortcut';
import { appWindow } from '@tauri-apps/api/window';
import { useKeyDownEvent } from '@solid-primitives/keyboard';

import './App.scss';
import SearchResult from './SearchResult';
import { StoreContext } from './store';
import { hide, toggle_main_window } from './invocations';
import { QueryMode } from './constants';

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

function Preview() {
  const [state, resource, selection] = useContext(StoreContext);

  return (
    <div class="preview-container">
      <Switch fallback={<div>No Preview</div>}>
        <Match when={selection()?.preview?.text}>
          {selection().preview.text}
        </Match>
      </Switch>
    </div>
  );
}

function App() {
  const [
    state,
    results,
    selection,
    {
      openCursor,
      openSelected,
      cursorUp,
      cursorDown,
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
    if (!exists) {
      await register('CommandOrControl+K', async () => {
        focus();
        set_search_string('');
        await toggle_main_window();
      });
    }
  });

  createEffect(async () => {
    const event = keyboardEvent();
    if (!event) return;

    const { key, shiftKey, ctrlKey, metaKey, altKey } = event;
    focus();

    switch (key) {
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
      case 'ArrowUp':
        return cursorUp();
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

  const mapped = () => {
    if (results.loading) {
      return [];
    }
    return results().map((r, i) => ({
      ...r,
      selected:
        i === state.cursor || state.cursor > (results?.latest?.length ?? 0),
    }));
  };

  return (
    <div ref={ref} class="search-container">
      <div class="handle draggable-area" data-tauri-drag-region />
      <div class="search-input-container draggable-area" data-tauri-drag-region>
        <input
          ref={inputRef}
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
        <Show when={!results.loading} fallback={loadingState}>
          <ul class="result-container">
            <For each={mapped()}>
              {(item, i) => (
                <SearchResult
                  selected={item.selected}
                  heading={item.heading}
                  subtext={item.subheading}
                />
              )}
            </For>
          </ul>
          <Preview />
        </Show>
      </div>
    </div>
  );
}

export default App;
