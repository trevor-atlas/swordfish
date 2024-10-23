import { invoke } from '@tauri-apps/api/core';

export async function hide() {
  await invoke('hide_main_window');
}
export async function show() {
  await invoke('show_main_window');
}
export async function toggle_main_window() {
  await invoke('toggle_main_window');
}

export async function toggle_settings_window() {
  await invoke('toggle_settings_window');
}

export async function hide_settings_window() {
  await invoke('hide_settings_window');
}

export async function show_settings_window() {
  await invoke('show_settings_window');
}
