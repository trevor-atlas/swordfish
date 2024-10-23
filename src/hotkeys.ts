import { register } from '@tauri-apps/plugin-global-shortcut';

await register('Escape', () => {
  console.log('Esc Shortcut triggered');
});
