import { createContext, JSX, onMount, useContext } from 'solid-js';
import { createStore } from 'solid-js/store';
import { LifecycleEvent, NUMERIC, QUERY_MODES } from './constants';
import { hide } from './invocations';
import { Nullable } from './types';
import { emit, listen } from '@tauri-apps/api/event';
import { QueryResult } from './types/QueryResult';
import { QueryResultItem } from './types/QueryResultItem';

type StoreState = {
  search_string: string;
  prev_search: string[];
  prev_search_index: number;
  touched: boolean;
  mode: number;
  cursor: number;
  queryResult: QueryResult;
};

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
    getSelectedResult(): Nullable<QueryResultItem>;
  },
];

const defaultState: StoreState = {
  search_string: '',
  prev_search: [],
  prev_search_index: 0,
  touched: false,
  mode: 0,
  cursor: 0,
  queryResult: { results: [] },
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
    listen<QueryResult>(LifecycleEvent.QueryResult, (data) => {
      if (!data || !data.payload) {
        return;
      }
      setState('queryResult', data.payload);
    });
    listen(LifecycleEvent.MainWindowHidden, () => {
      console.log('main window hidden');
      resetAndHide();
    });
  });

  function setSearchString(str: string) {
    if (!str) {
      setState(() => ({ search_string: str, touched: false }));
    } else {
      setState(() => ({ search_string: str, touched: true }));
    }
    emit(LifecycleEvent.Query, {
      mode: QUERY_MODES[state.mode],
      search_string: str,
    });
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
        const newMode = s - 1 < 0 ? QUERY_MODES.length - 1 : s - 1;
        emit(LifecycleEvent.Query, {
          mode: QUERY_MODES[newMode],
          search_string: state.search_string,
        });
        return newMode;
      }

      const newMode = s + 1 > QUERY_MODES.length - 1 ? 0 : s + 1;
      emit(LifecycleEvent.Query, {
        mode: QUERY_MODES[newMode],
        search_string: state.search_string,
      });
      return newMode;
    });
  }

  function setCursor(cursor: number) {
    setState(() => ({ cursor }));
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
        emit(LifecycleEvent.Query, {
          mode: QUERY_MODES[state.mode],
          search_string: search,
        });
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
