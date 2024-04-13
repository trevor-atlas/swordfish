export const CHAT = 'Chat';
export const SEARCH = 'Search';
export const SCRIPTS = 'Scripts';
export const BROWSER_HISTORY = 'BrowserHistory';
export const QUERY_MODES = [SEARCH, BROWSER_HISTORY, SCRIPTS, CHAT] as const;
export type QueryMode = (typeof QUERY_MODES)[number];

export const NUMERIC = /\d+/;
