import {
  For,
  Show,
  useContext,
} from 'solid-js';
import { appWindow } from '@tauri-apps/api/window';

import './App.scss';
import SearchResult from './SearchResult';
import { StoreContext } from './store';
import { QueryMode } from './constants';
import Preview from './components/Preview';
import QueryResultList from './components/QueryResultList';
import { useInputHandler } from './hooks/useInputHandler';

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
        {(item) => (
          <div class="action active">
            <span>{item.title}</span>
          </div>
        )}
      </For>
    </div>
  );
}

const u = await appWindow.onFocusChanged(async ({ payload: focused }) => {
  // if (!focused) {
  //   await hide();
  // }
});


function App() {
  const [
    state,
    {
      set_search_string,
    },
  ] = useContext(StoreContext);

  let inputRef!: HTMLInputElement;
  let ref;

  useInputHandler(() => {
    if (inputRef) {
      inputRef.focus();
    }
  });
  

  // onMount(async () => {
    // const exists = await isRegistered('CommandOrControl+K');
    // if (!exists) {
    // await register('CommandOrControl+K', async () => {
    //   focus();
    //   set_search_string('');
    //   setCursor(0);
    //   await toggle_main_window();
    // });
    // }
  // })


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
