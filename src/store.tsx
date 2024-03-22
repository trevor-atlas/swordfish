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

type None = null | undefined;
type Nullable<T> = T | None;
const isSome = <T,>(a: Nullable<T>): a is T => typeof a != null;
const isNone = <T,>(a: Nullable<T>): a is None => !isSome(a);

type Store = [
  StoreState,
  {
    setSearchString(str: string): void;
    nextSearchMode(mode: QueryModes): void;
    prevSearchMode(mode: QueryModes): void;
    setCursor(cursor: number): void;
    cursorUp(): void;
    cursorDown(): void;
    resetAndHide(): void;
    getSelectedResult(): Nullable<QueryResultEntry>;
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
  {},
] as unknown as Store);

export function useStore() {
  return useContext(StoreContext);
}

export function StoreProvider(props: { children: JSX.Element }) {
  const [state, setState] = createStore<StoreState>(defaultState);

  onMount(() => {
    listen('query', (data) => {
      if (!data || !data.payload) return;
      console.log('QueryResult:', data.payload);
      setState('queryResult', data.payload);
    });
  });

  function setSearchString(str: string) {
    setState('search_string', () => str);
    emit('query', { mode: state.mode, search_string: str });
  }

  async function resetAndHide() {
    await hide();
    setSearchString('');
    setState('queryResult', () => ({ inline_result: '', results: [] }));
  }

  function setSearchMode(isAdvancing: boolean = true) {
    setState('mode', (s) => {
      if (!isAdvancing) {
        for (const mode in Object.values(QueryMode).reverse()) {
          if (mode === s) continue;
          return mode as QueryModes;
        }
      }
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

  function getSelectedResult(key?: string) {
    if (!state.queryResult.results.length) {
      return;
    }
    if (key && NUMERIC.test(key)) {
      const parsedIdx = parseInt(key, 10);
      const keyIndex =
        parsedIdx > state.queryResult.results.length
          ? state.queryResult.results.length - 1
          : parsedIdx - 1;
      return state.queryResult.results[keyIndex];
    }
    const targetIdx = state.cursor;
    const keyIndex =
      targetIdx > state.queryResult.results.length
        ? state.queryResult.results.length - 1
        : targetIdx;
    console.log('select item', state.queryResult.results[keyIndex]);
    return state.queryResult.results[keyIndex];
  }

  const store: Store = [
    state,
    {
      setSearchString,
      nextSearchMode: () => setSearchMode(),
      prevSearchMode: () => setSearchMode(false),
      getSelectedResult,
      setCursor,
      cursorDown,
      cursorUp,
      resetAndHide,
    },
  ];

  return (
    <StoreContext.Provider value={store}>
      {props.children}
    </StoreContext.Provider>
  );
}
