import { useKeyDownEvent } from '@solid-primitives/keyboard';
import { createEffect } from 'solid-js';
import { toggle_settings_window } from './invocations';

export default function Settings() {
  const keyboardEvent = useKeyDownEvent();

  createEffect(async () => {
    const event = keyboardEvent();
    if (!event) return;
    const { key, shiftKey, ctrlKey, metaKey, altKey } = event;

    console.log(key);
    switch (key) {
      case 'Escape': {
        return toggle_settings_window();
      }
    }
  });

  return <>Settings!</>;
}
