import { invoke } from '@tauri-apps/api';
import { Query, QueryResponse } from './types';

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

export async function get_query_result(query: Query): Promise<QueryResponse> {
  return invoke('get_query_result', { query });
}
