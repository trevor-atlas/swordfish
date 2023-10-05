import { register } from '@tauri-apps/api/globalShortcut';

await register('Escape', () => {
  console.log('Esc Shortcut triggered');
});
