import { For, Show, createResource, createSignal, onMount } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import { register } from '@tauri-apps/api/globalShortcut';
import { isRegistered } from '@tauri-apps/api/globalShortcut';
import { unregister } from '@tauri-apps/api/globalShortcut';
import { unregisterAll } from '@tauri-apps/api/globalShortcut';
await unregisterAll();

import './App.css';
import SearchResult from './SearchResult';

const QueryMode = {
  Clipboard: 'Clipboard',
  BrowserHistory: 'BrowserHistory',
  Files: 'Files',
  Scripts: 'Scripts',
  Chat: 'Chat',
} as const;

type Query = {
  search_string: string;
  mode: keyof typeof QueryMode;
};

async function getData(query: Query): Promise<any[]> {
  const data = await invoke<any[]>('get_query_result', {
    query,
  });
  return data;
}

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

// (async () => {
//   // const exists = await isRegistered('ESC');
//   // console.log({ exists });
//   // if (!exists) {
//   await register('ESCAPE', () => {
//     console.log('Esc Shortcut triggered');
//   });
//   await register('CommandOrControl+Shift+C', () => {
//     console.log('Shortcut triggered');
//   });
//   // }
// })();

function App() {
  const [input, setInput] = createSignal('');
  const [mode, setMode] = createSignal(QueryMode.Files);
  const query = () => ({ search_string: input(), mode: mode() });
  const [data] = createResource<any[], Query>(query, getData);

  const unlisten = listen('keypress', (event) => {
    console.log(event);
  });

  let ref;
  let inputRef;

  return (
    <div ref={ref} class="search-container draggable-area">
      <div class="search-input-container" data-tauri-drag-region>
        <input
          ref={inputRef}
          autofocus={true}
          type="text"
          spellcheck={false}
          class="search-input"
          value={input()}
          onInput={(e) => {
            setInput(e.currentTarget.value);
          }}
        />
      </div>
      <div class="details-container">
        <div class="result-container">
          <ul>
            <Show when={!data.loading} fallback={loadingState}>
              <For each={data()}>
                {(item) => (
                  <SearchResult
                    heading={item.heading}
                    subtext={item.subheading}
                  />
                )}
              </For>
            </Show>
          </ul>
        </div>
        <div class="preview-container">
          {/* <img src="https://placehold.it/300/300"/> */}
        </div>
      </div>
    </div>
  );
}

export default App;
