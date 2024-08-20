import { QueryMode } from './types/QueryMode';
import { SFEvent } from './types/SFEvent';

export const CHAT: QueryMode = 'Chat';
export const SEARCH: QueryMode = 'Search';
export const SCRIPTS: QueryMode = 'Scripts';
export const BROWSER_HISTORY: QueryMode = 'BrowserHistory';
export const QUERY_MODES: QueryMode[] = [
  SEARCH,
  BROWSER_HISTORY,
  SCRIPTS,
  CHAT,
] as const;

export const NUMERIC = /\d+/;

export const LifecycleEvent = Object.freeze({
  MainWindowShown: 'MainWindowShown',
  MainWindowHidden: 'MainWindowHidden',
  SettingsWindowShown: 'SettingsWindowShown',
  SettingsWindowHidden: 'SettingsWindowHidden',
  MainWindowResized: 'MainWindowResized',
  Query: 'Query',
  QueryResult: 'QueryResult',
}) satisfies Record<SFEvent, SFEvent>;
