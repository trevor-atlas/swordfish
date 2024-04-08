export const CHAT = 'Chat';
export const SEARCH = 'Search';
export const SCRIPTS = 'Scripts';
export const QUERY_MODES = [SEARCH, CHAT, SCRIPTS] as const;
export type QueryMode = (typeof QUERY_MODES)[number];

export const NUMERIC = /\d+/;
