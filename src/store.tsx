import {
  createContext,
  createEffect,
  createResource,
  JSX,
  useContext,
} from 'solid-js';
import { createStore } from 'solid-js/store';
import { NUMERIC, QueryMode } from './constants';
import { Query, QueryModes, QueryResult, QueryResultEntry } from './types';
import { get_query_result } from './invocations';

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

  const [resource] = createResource<QueryResult, Query>(
    () => ({
      search_string: state.search_string,
      mode: state.mode,
    }),
    get_query_result,
    {
      initialValue: {
        inline_result: '',
        results: [],
      },
    },
  );

  createEffect(() => {
    const data = resource();
    if (!data) return;
    setState('queryResult', data);
    const keyIndex =
      state.cursor > data.results.length
        ? data.results.length - 1
        : state.cursor;
    setCursor(keyIndex);
  });

  const hasQueryResults = (r: any) =>
    Boolean(r.state === 'ready' && r.latest.length);

  function set_search_string(str: string) {
    setState('search_string', () => str);
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
      selection: resource.latest.results[cursor],
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
        if (!hasQueryResults(resource)) {
          return s.cursor - 1 < 0 ? s.cursor : s.cursor - 1;
        }

        return s.cursor - 1 < 0
          ? resource.latest.results.length - 1
          : s.cursor - 1;
      })();
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }
  function cursorDown() {
    setState((s) => {
      const cursor = (() => {
        if (!hasQueryResults(resource)) return 0;

        return s.cursor < resource.latest.results.length - 1 ? s.cursor + 1 : 0;
      })();
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }
  function openSelected(key?: string) {
    if (!hasQueryResults(resource) || !key || !NUMERIC.test(key)) return;
    const parsedIdx = parseInt(key, 10);
    const keyIndex =
      parsedIdx > resource.latest.results.length
        ? resource.latest.results.length - 1
        : parsedIdx - 1;
    console.log('select item', resource().results[keyIndex]);
  }
  function openCursor() {
    if (!hasQueryResults(resource)) return;

    const targetIdx = state.cursor;
    const keyIndex =
      targetIdx > resource.latest.results.length
        ? resource.latest.results.length - 1
        : targetIdx;
    console.log('cursor selected', resource.latest.results[keyIndex]);
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
