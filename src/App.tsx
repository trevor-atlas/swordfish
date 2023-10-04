import { For, Show, createResource, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SearchResult from "./SearchResult";

async function getData(query: string): Promise<any[]> {
  const data = await invoke<any[]>("getResults", {
    query: { search_string: query, kind: "Placeholder" },
  });
  await new Promise((res) => setTimeout(res, 100000));
  return data;
}

const loadingState = (
  <For each={[1, 2, 3, 4, 5, 6]}>
    {() => (
      <span
        class="shimmerBG"
        style={{
          width: "100%",
          height: "5rem",
        }}
      ></span>
    )}
  </For>
);

function App() {
  const [input, setInput] = createSignal("");
  const [data] = createResource<any[], string>(input, getData);

  let ref;
  let inputRef;

  return (
    <div ref={ref} class="search-container draggable-area">
      <div class="search-input-container">
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
