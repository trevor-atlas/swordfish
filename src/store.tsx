import {
  Accessor,
  createContext,
  createEffect,
  createResource,
  createSignal,
  JSX,
  Resource,
  Signal,
} from 'solid-js';
import { createStore } from 'solid-js/store';
import { NUMERIC, QueryMode } from './constants';
import { Query, QueryModes, QueryResult, Result, Results } from './types';
import { invoke } from '@tauri-apps/api';
import { get_query_result } from './invocations';

const voidFN = () => {};

type StoreState = Query & { cursor: number };
type QueryResults = Resource<QueryResult>;

type Store = [
  StoreState,
  Resource<QueryResult[]>,
  Accessor<QueryResult | null>,
  {
    set_search_string(str: string): void;
    set_search_mode(mode: QueryModes): void;
    setCursor(cursor: number): void;
    cursorUp(): void;
    cursorDown(): void;
    openSelected(key: `${number}`): void;
    openCursor(): void;
  },
];

const defaultState = {
  search_string: '',
  mode: QueryMode.Files,
  cursor: 0,
};

const pending = Object.assign(() => [] as any[], {
  state: 'pending',
  loading: true,
  error: false,
  latest: [] as QueryResult[],
}) as Resource<QueryResult[]>;

export const StoreContext = createContext<Store>([
  defaultState,
  pending,
  () => null,
  {
    set_search_string: voidFN,
    set_search_mode: voidFN,
    setCursor: voidFN,
    cursorUp: voidFN,
    cursorDown: voidFN,
    openSelected: voidFN,
    openCursor: voidFN,
  },
]);

export function StoreProvider(props: { children: JSX.Element }) {
  const [state, setState] = createStore<StoreState>(defaultState);
  const query = () => ({
    search_string: state.search_string,
    mode: state.mode,
  });

  const [resource] = createResource<Results[], Query>(query, get_query_result, {
    initialValue: [],
  });

  const [selection, setSelection] = createSignal<QueryResult | null>(null);

  createEffect(() => {
    const data = resource();
    if (!data) return;
    const keyIndex =
      state.cursor > data.length ? data.length - 1 : state.cursor;
    setSelection(data[keyIndex]);
  });

  const hasQueryResults = (r: any) =>
    Boolean(r.state === 'ready' && r.latest.length);

  const store: Store = [
    state,
    resource,
    selection,
    {
      set_search_string(str: string) {
        setState('search_string', () => str);
      },
      set_search_mode(mode: QueryModes) {
        setState('mode', () => mode);
      },
      setCursor(cursor: number) {
        setState((s) => ({
          selection: resource.latest[cursor],
          cursor,
        }));
      },
      cursorUp() {
        setState((s) => {
          if (!hasQueryResults(resource)) {
            return {
              cursor: s.cursor - 1 < 0 ? s.cursor : s.cursor - 1,
            };
          }

          const cursor =
            s.cursor - 1 < 0 ? resource.latest.length - 1 : s.cursor - 1;

          return { cursor };
        });
      },
      cursorDown() {
        setState((s) => {
          if (!hasQueryResults(resource)) return { cursor: 0 };

          const cursor =
            s.cursor < resource.latest.length - 1 ? s.cursor + 1 : 0;

          return { cursor };
        });
      },
      openSelected(key?: string) {
        if (!hasQueryResults(resource) || !key || !NUMERIC.test(key)) return;
        const parsedIdx = parseInt(key, 10);
        const keyIndex =
          parsedIdx > resource.latest.length
            ? resource.latest.length - 1
            : parsedIdx - 1;
        console.log('select item', resource()[keyIndex]);
      },
      openCursor() {
        if (!hasQueryResults(resource)) return;

        const targetIdx = state.cursor;
        const keyIndex =
          targetIdx > resource.latest.length
            ? resource.latest.length - 1
            : targetIdx;
        console.log('cursor selected', resource.latest[keyIndex]);
      },
    },
  ];

  return (
    <StoreContext.Provider value={store}>
      {props.children}
    </StoreContext.Provider>
  );
}
