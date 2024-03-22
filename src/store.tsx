import {
  createContext,
  createEffect,
  createResource,
  JSX,
  onMount,
  useContext,
} from 'solid-js';
import { createStore } from 'solid-js/store';
import { NUMERIC, QUERY_MODES, QueryMode } from './constants';
import { Query, QueryResult, QueryResultEntry } from './types';
import { get_query_result } from './invocations';
import { hide } from './invocations';
import { open } from '@tauri-apps/api/shell';

import { emit, listen } from '@tauri-apps/api/event';

const voidFN = () => {};

type StoreState = {
  search_string: string;
  prev_search: string[];
  prev_search_index: number;
  touched: boolean;
  mode: number;
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
    nextSearchMode(): void;
    prevSearchMode(): void;
    setCursor(cursor: number): void;
    cursorUp(): void;
    cursorDown(): void;
    resetAndHide(): void;
    getSelectedResult(): Nullable<QueryResultEntry>;
  },
];

const defaultState = {
  search_string: '',
  prev_search: [],
  prev_search_index: 0,
  touched: false,
  mode: 0,
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
    listen('appwindow:hidden', () => {
      resetAndHide();
    });
  });

  function setSearchString(str: string) {
    if (!str) {
      setState(() => ({ search_string: str, touched: false }));
      return;
    }
    setState(() => ({ search_string: str, touched: true }));
    emit('query', { mode: QUERY_MODES[state.mode], search_string: str });
  }

  async function resetAndHide() {
    await hide();
    setState((s) => ({
      search_string: '',
      touched: false,
      prev_search: [...s.prev_search, s.search_string],
      prev_search_index: 0,
      queryResult: { inline_result: '', results: [] },
    }));
  }

  function setSearchMode(isAdvancing: boolean = true) {
    setState('mode', (s) => {
      if (!isAdvancing) {
        return s - 1 < 0 ? QUERY_MODES.length - 1 : s - 1;
      }
      return s + 1 > QUERY_MODES.length - 1 ? 0 : s + 1;
    });
  }

  function set_search_mode(mode: QueryMode) {
    setState('mode', () => QUERY_MODES.indexOf(mode));
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
      // if (!query) {
      //   return s.cursor - 1 < 0 ? s.cursor : s.cursor - 1;
      // }
      if (!s.touched && s.cursor === 0) {
        const idx = s.prev_search_index + 1;
        const search =
          s.prev_search[idx % s.prev_search.length] || s.search_string;
        emit('query', { mode: QUERY_MODES[state.mode], search_string: search });
        return {
          prev_search_index: idx,
          search_string: search,
        };
      }
      if (!s.queryResult.results.length) {
        return s;
      }

      const cursor =
        s.cursor - 1 < 0 ? state.queryResult.results.length - 1 : s.cursor - 1;
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }

  function cursorDown() {
    setState((s) => {
      if (!state.queryResult.results.length) {
        return s;
      }

      const cursor =
        s.cursor < state.queryResult.results.length - 1 ? s.cursor + 1 : 0;
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
