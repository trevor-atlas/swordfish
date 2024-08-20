import { useCallback } from 'react';
import { useInputHandler } from './hooks/useInputHandler';
import { toggle_settings_window } from './invocations';

export default function Settings() {
  useInputHandler(
    useCallback(async (event) => {
      if (!event) return;
      const { key, shiftKey, ctrlKey, metaKey, altKey } = event;
      switch (key) {
        case 'Escape': {
          await toggle_settings_window();
        }
      }
    }, []),
  );

  return <>Settings!</>;
}
