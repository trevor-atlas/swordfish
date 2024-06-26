import { appWindow } from '@tauri-apps/api/window';
import { For, Match, Show, Switch, useContext } from 'solid-js';
import './App.scss';
import { Chat } from './components/Chat';
import Preview from './components/Preview';
import QueryResultList from './components/QueryResultList';
import { BROWSER_HISTORY, CHAT, QUERY_MODES, SEARCH } from './constants';
import { useInputHandler } from './hooks/useInputHandler';
import { StoreContext, useStore } from './store';

const loadingState = (
  <div
    style={{
      width: '100%',
      height: '5rem',
    }}
  />
);

function ActionSelector() {
  const [store] = useStore();
  const actions = QUERY_MODES.map((str) => ({
    title: str,
  }));
  return (
    <div class="action-selector">
      <For each={actions}>
        {(item) => (
          <div
            class={`${QUERY_MODES[store.mode] === item.title && 'active'} action `}
          >
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
  const [state, { setSearchString }] = useContext(StoreContext);

  let inputRef!: HTMLInputElement;

  useInputHandler(() => {
    if (inputRef) {
      setTimeout(() => inputRef.focus(), 0);
    }
  });

  return (
    <div class="search-container">
      <div class="handle draggable-area" data-tauri-drag-region />
      <div class="search-input-container draggable-area" data-tauri-drag-region>
        <input
          ref={inputRef}
          onKeyDown={(event) => {
            if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
              event.preventDefault();
            }
          }}
          autofocus={true}
          type="text"
          spellcheck={false}
          class="search-input"
          value={state.search_string}
          onInput={(e) => {
            setSearchString(e.currentTarget.value);
          }}
        />
      </div>
      <ActionSelector />
      <Switch fallback={<div>No Preview</div>}>
        <Match
          when={
            QUERY_MODES[state.mode] === SEARCH ||
            QUERY_MODES[state.mode] === BROWSER_HISTORY
          }
        >
          <Show
            when={state.queryResult.results && state.queryResult.results.length}
            fallback={loadingState}
          >
            <div class="flex flex-row">
              <QueryResultList />
              <Preview />
            </div>
          </Show>
        </Match>
        <Match when={QUERY_MODES[state.mode] === CHAT}>
          <Chat />
        </Match>
      </Switch>
    </div>
  );
}

function Footer() {
  return (
    <div class="flex flex-row border-t border-ui-border bg-ui-bg max-h-6.5 min-h-6.5 h-6.5 items-center justify-center overflow-hidden px-4 pt-px">
      :O
    </div>
  );
}

export default App;
