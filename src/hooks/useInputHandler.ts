import { createEffect, useContext } from 'solid-js';
import { hide, toggle_settings_window } from '../invocations';
import { StoreContext } from '../store';
import { emit } from '@tauri-apps/api/event';
import { useKeyDownEvent } from '@solid-primitives/keyboard';
import { QueryMode } from '../constants';

function switchMode(mode: QueryMode) {
  switch (mode) {
    case QueryMode.Chat:
      return QueryMode.Search;
    case QueryMode.Search:
      return QueryMode.Chat;
    default:
      return mode;
  }
}

export function useInputHandler(onPress: () => void) {
  const keyboardEvent = useKeyDownEvent();
  const [
    state,
    {
      openCursor,
      openSelected,
      cursorUp,
      cursorDown,
      set_search_string,
      switch_search_mode,
    },
  ] = useContext(StoreContext);

  createEffect(async () => {
    const event = keyboardEvent();
    if (!event) return;
    onPress();

    const { key, shiftKey, ctrlKey, metaKey, altKey, location } = event;
    const hasModifier = shiftKey || ctrlKey || metaKey || altKey;
    focus();

    console.log('keypress', {
      key,
      shiftKey,
      ctrlKey,
      metaKey,
      altKey,
      location,
    });

    if (hasModifier) {
      if (location === 1) console.log(`left `);
      if (location === 2) console.log(`right `);
    }

    switch (key) {
      case 'Tab': {
        event.preventDefault();
        event.stopPropagation();
        return switch_search_mode();
        break;
      }
      case ',': {
        if (metaKey) {
          return toggle_settings_window();
        }
        break;
      }
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9': {
        if (metaKey || ctrlKey) {
          openSelected(key);
        }
        return;
      }
      case 'ArrowUp': {
        if (!state.search_string && state.cursor === 0) {
          // show last command/search string
          return;
        } else {
          return cursorUp();
        }
      }
      case 'ArrowDown':
        return cursorDown();
      case 'Enter': {
        return openCursor();
      }
      case 'Escape': {
        await hide();
        set_search_string('');
        return;
      }
    }
  });
}
