import { QueryMode } from './types/QueryMode';

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
