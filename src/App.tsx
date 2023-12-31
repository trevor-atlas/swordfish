import { For, Match, Show, Switch, useContext } from 'solid-js';
import { appWindow } from '@tauri-apps/api/window';
import './App.scss';
import { StoreContext, useStore } from './store';
import { QueryMode } from './constants';
import Preview from './components/Preview';
import QueryResultList from './components/QueryResultList';
import { useInputHandler } from './hooks/useInputHandler';
import { Chat } from './components/Chat';

const loadingState = (
  <For each={[1, 2, 3, 4, 5, 6]}>
    {() => (
      <div
        class="shimmer-bg"
        style={{
          width: '100%',
          height: '5rem',
        }}
      />
    )}
  </For>
);

function ActionSelector() {
  const [store] = useStore();
  const actions = Object.keys(QueryMode).map((str) => ({
    title: str,
  }));
  return (
    <div class="action-selector">
      <For each={actions}>
        {(item) => (
          <div class={`${store.mode === item.title && 'active'} action `}>
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
  const [state, { set_search_string }] = useContext(StoreContext);

  let inputRef!: HTMLInputElement;
  let ref;

  useInputHandler(() => {
    if (inputRef) {
      inputRef.focus();
    }
  });

  return (
    <div ref={ref} class="search-container">
      <div class="handle draggable-area" data-tauri-drag-region />
      <div class="search-input-container draggable-area" data-tauri-drag-region>
        <input
          ref={inputRef}
          onKeyDown={(event) => {
            if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
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
        <div>{state.queryResult.inline_result}</div>
      </div>
      <ActionSelector />
      <Switch fallback={<div>No Preview</div>}>
        <Match when={state.mode === QueryMode.Chat}>
          <Chat />
        </Match>
        <Match when={state.mode === QueryMode.Search}>
          <div class="details-container">
            <Show
              when={
                state.queryResult.results && state.queryResult.results.length
              }
              fallback={loadingState}
            >
              <QueryResultList />
              <Preview />
            </Show>
          </div>
        </Match>
      </Switch>
    </div>
  );
}

export default App;
