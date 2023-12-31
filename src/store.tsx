import {
  createContext,
  createEffect,
  createResource,
  JSX,
  onMount,
  useContext,
} from 'solid-js';
import { createStore } from 'solid-js/store';
import { NUMERIC, QueryMode } from './constants';
import { Query, QueryModes, QueryResult, QueryResultEntry } from './types';
import { get_query_result } from './invocations';
import { hide } from './invocations';
import { open } from '@tauri-apps/api/shell';

import { emit, listen } from '@tauri-apps/api/event';

const voidFN = () => {};

type StoreState = Query & {
  cursor: number;
  selection: QueryResult | null;
  queryResult: QueryResult;
};

type Store = [
  StoreState,
  {
    set_search_string(str: string): void;
    set_search_mode(mode: QueryModes): void;
    switch_search_mode(mode: QueryModes): void;
    setCursor(cursor: number): void;
    cursorUp(): void;
    cursorDown(): void;
    openSelected(key: `${number}`): void;
    openCursor(): void;
  },
];

const defaultState = {
  search_string: '',
  mode: QueryMode.Search,
  queryResult: { inline_result: '', results: [] },
  cursor: 0,
  selection: null,
};

export const StoreContext = createContext<Store>([
  defaultState,
  {
    set_search_string: voidFN,
    set_search_mode: voidFN,
    switch_search_mode: voidFN,
    setCursor: voidFN,
    cursorUp: voidFN,
    cursorDown: voidFN,
    openSelected: voidFN,
    openCursor: voidFN,
  },
]);

export function useStore() {
  return useContext(StoreContext);
}

export function StoreProvider(props: { children: JSX.Element }) {
  const [state, setState] = createStore<StoreState>(defaultState);

  onMount(() => {
    listen('query', (data) => {
      if (!data || !data.payload) return;
      setState('queryResult', data.payload);
    });
  });

  function set_search_string(str: string) {
    setState('search_string', () => str);
    emit('query', { mode: state.mode, search_string: str });
  }

  function switch_search_mode() {
    setState('mode', (s) => {
      for (const mode in QueryMode) {
        if (mode === s) continue;
        return mode as QueryModes;
      }
      return s;
    });
  }

  function set_search_mode(mode: QueryModes) {
    setState('mode', () => mode);
  }

  function setCursor(cursor: number) {
    //@ts-ignore
    setState(() => ({
      selection: state.queryResult.results[cursor],
      cursor,
    }));
  }
  // teehee dom logic in state :D
  function scrollToCursorPosition(cursor: number) {
    const ref = document.querySelector(`.query-result-${cursor}`);
    if (ref) {
      ref.scrollIntoView({
        behavior: 'auto',
        block: 'center',
      });
    }
  }

  function cursorUp() {
    setState((s) => {
      const cursor = (() => {
        // if (!query) {
        //   return s.cursor - 1 < 0 ? s.cursor : s.cursor - 1;
        // }

        return s.cursor - 1 < 0
          ? state.queryResult.results.length - 1
          : s.cursor - 1;
      })();
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }

  function cursorDown() {
    setState((s) => {
      const cursor = (() => {
        if (!state.queryResult.results.length) return 0;

        return s.cursor < state.queryResult.results.length - 1
          ? s.cursor + 1
          : 0;
      })();
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }

  function openSelected(key?: string) {
    if (!state.queryResult.results.length || !key || !NUMERIC.test(key)) return;
    const parsedIdx = parseInt(key, 10);
    const keyIndex =
      parsedIdx > state.queryResult.results.length
        ? state.queryResult.results.length - 1
        : parsedIdx - 1;
    console.log('select item', state.queryResult.results[keyIndex]);
  }

  function openCursor() {
    if (!state.queryResult.results.length) return;

    const targetIdx = state.cursor;
    const keyIndex =
      targetIdx > state.queryResult.results.length
        ? state.queryResult.results.length - 1
        : targetIdx;
    open(state.queryResult.results[keyIndex].subheading);
    hide();
    console.log('cursor selected', state.queryResult.results[keyIndex]);
  }

  const store: Store = [
    state,
    {
      set_search_string,
      set_search_mode,
      switch_search_mode,
      setCursor,
      openCursor,
      openSelected,
      cursorDown,
      cursorUp,
    },
  ];

  return (
    <StoreContext.Provider value={store}>
      {props.children}
    </StoreContext.Provider>
  );
}
