import { useKeyDownEvent } from '@solid-primitives/keyboard';
import { writeText } from '@tauri-apps/api/clipboard';
import { createEffect, useContext } from 'solid-js';
import { open } from '@tauri-apps/api/shell';
import { toggle_settings_window } from '../invocations';
import { StoreContext } from '../store';
import { FILE_RESULT, Nullable, QueryResult, QueryResultEntry } from '../types';

export function useInputHandler(onPress: () => void) {
  const keyboardEvent = useKeyDownEvent();
  const [
    state,
    {
      cursorUp,
      cursorDown,
      setSearchString,
      getSelectedResult,
      resetAndHide,
      nextSearchMode,
      prevSearchMode,
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

    async function openResult(result: Nullable<QueryResultEntry>) {
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
        default: {
          await open(result.subheading);
          await resetAndHide();
          break;
        }
      }
    }

    if (hasModifier) {
      if (location === 1) console.log(`left `);
      if (location === 2) console.log(`right `);
    }

    switch (key) {
      case 'Tab': {
        event.preventDefault();
        event.stopPropagation();
        if (shiftKey) {
          return prevSearchMode();
        }
        nextSearchMode();
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
          const value = getSelectedResult()?.subheading;
          if (value) {
            open(value);
          }
        }
        return;
      }
      case 'c':
      case 'C':
        if ((metaKey || ctrlKey) && shiftKey) {
          console.log('I should clipboard the current value in the cursor');
          const value = getSelectedResult()?.subheading;
          if (value) {
            await writeText(value);
          }
        }
        break;
      case 'ArrowUp': {
        // if (!state.search_string && state.cursor === 0) {
        // show last command/search string
        // return;
        // } else {
        return cursorUp();
        // }
      }
      case 'ArrowDown':
        return cursorDown();
      case 'Enter': {
        const value = getSelectedResult();
        openResult(value);
        break;
      }
      case 'Escape': {
        await resetAndHide();
        return;
      }
    }
  });
}
