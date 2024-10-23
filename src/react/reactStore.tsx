import { create } from 'zustand';
import { LifecycleEvent, NUMERIC, QUERY_MODES } from '../constants';
import { hide } from '../invocations';
import { emit, listen } from '@tauri-apps/api/event';
import { QueryResult } from '../types/QueryResult';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Nullable, FILE_RESULT, CALCULATOR_RESULT } from '../types';
import { QueryResultItem } from '../types/QueryResultItem';
import { open } from '@tauri-apps/plugin-shell';

type ApplicationState = {
  search_string: string;
  prev_search: string[];
  prev_search_index: number;
  touched: boolean;
  mode: number;
  cursor: number;
  queryResult: QueryResult;
};

type Store = ApplicationState & {
  init(): void;
  setSearchString(str: string): void;
  nextSearchMode(): void;
  prevSearchMode(): void;
  setCursor(cursor: number): void;
  cursorUp(): void;
  cursorDown(): void;
  resetAndHide(): Promise<void>;
};

const defaultState: ApplicationState = {
  search_string: '',
  prev_search: [],
  prev_search_index: 0,
  touched: false,
  mode: 0,
  cursor: 0,
  queryResult: { results: [] },
};

export const useStore = create<Store>()((set, get) => {
  function init() {
    listen<QueryResult>(LifecycleEvent.QueryResult, (data) => {
      if (!data || !data.payload) {
        return;
      }
      set({ queryResult: data.payload });
    });
    listen(LifecycleEvent.MainWindowHidden, () => {
      console.log('main window hidden');
      resetAndHide();
    });
  }

  function setSearchString(str: string) {
    set((s) => ({ search_string: str, touched: s.touched || !!str }));
    emit(LifecycleEvent.Query, {
      mode: QUERY_MODES[get().mode],
      search_string: str,
    });
  }

  async function resetAndHide() {
    await hide();
    set(({ prev_search, search_string }) => ({
      search_string: '',
      touched: false,
      prev_search: search_string
        ? [...prev_search, search_string]
        : prev_search,
      prev_search_index: 0,
      queryResult: { inline_result: '', results: [] },
      cursor: 0,
    }));
  }

  function setSearchMode(isAdvancing: boolean = true) {
    set(({ mode, search_string }) => {
      const newMode = isAdvancing
        ? (mode + 1) % QUERY_MODES.length
        : (mode - 1 + QUERY_MODES.length) % QUERY_MODES.length;
      emit(LifecycleEvent.Query, {
        mode: QUERY_MODES[newMode],
        search_string,
      });
      return { mode: newMode };
    });
  }

  function setCursor(cursor: number) {
    set({ cursor });
  }

  function cursorUp() {
    set((s) => {
      if (!s.touched && s.cursor === 0 && s.prev_search.length) {
        const idx = (s.prev_search_index + 1) % s.prev_search.length;
        const search = s.prev_search[idx];
        emit(LifecycleEvent.Query, {
          mode: QUERY_MODES[s.mode],
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
        (s.cursor - 1 + s.queryResult.results.length) %
        s.queryResult.results.length;
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }

  function cursorDown() {
    set((s) => {
      if (!s.touched && s.cursor === 0 && s.prev_search.length) {
        const idx =
          (s.prev_search_index - 1 + s.prev_search.length) %
          s.prev_search.length;
        const search =
          s.prev_search[idx % s.prev_search.length] || s.search_string;
        emit(LifecycleEvent.Query, {
          mode: QUERY_MODES[s.mode],
          search_string: search,
        });
        return s;
      }

      if (!s.queryResult.results.length) {
        return s;
      }

      const cursor = (s.cursor + 1) % s.queryResult.results.length;
      scrollToCursorPosition(cursor);
      return { cursor };
    });
  }

  return {
    ...defaultState,
    init,
    setSearchString,
    nextSearchMode: () => setSearchMode(),
    prevSearchMode: () => setSearchMode(false),
    setCursor,
    cursorDown,
    cursorUp,
    resetAndHide,
  };
});

export function getSelectedResult(key?: number) {
  const s = useStore.getState();
  if (!s.queryResult.results.length) {
    return;
  }
  if (key) {
    const keyIndex =
      key > s.queryResult.results.length
        ? s.queryResult.results.length - 1
        : key - 1 || 0;
    console.log('select item', s.queryResult.results[keyIndex]);
    return s.queryResult.results[keyIndex];
  }
  const targetIdx = s.cursor;
  const keyIndex =
    targetIdx > s.queryResult.results.length
      ? s.queryResult.results.length - 1
      : targetIdx;
  console.log('select item', s.queryResult.results[keyIndex]);
  return s.queryResult.results[keyIndex];
}

function scrollToCursorPosition(cursor: number) {
  const ref = document.querySelector(`.query-result-${cursor}`);
  if (ref) {
    ref.scrollIntoView({
      behavior: 'auto',
      block: 'center',
    });
  }
}

export async function openResult(result: Nullable<QueryResultItem>) {
  const { resetAndHide } = useStore.getState();
  if (!result) {
    console.log('selection is invalid!?', result);
    return;
  }
  switch (result.type) {
    case FILE_RESULT: {
      await open(result.subheading);
      await resetAndHide();
      break;
    }
    case CALCULATOR_RESULT: {
      await writeText(result.heading);
      await resetAndHide();
      break;
    }
    default: {
      await open(result.subheading);
      await resetAndHide();
      break;
    }
  }
}
