import { For, createSignal } from 'solid-js';
import SearchResult from '../SearchResult';
import { useStore } from '../store';

export default function QueryResultList() {
  const [state] = useStore();
  return (
    <ul class="result-container">
      <For each={state.queryResult}>
        {(item, index) => (
          <SearchResult
            index={index()}
            heading={item.heading}
            subtext={item.subheading}
          />
        )}
      </For>
    </ul>
  );
}
