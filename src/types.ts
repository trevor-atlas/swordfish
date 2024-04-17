import { QueryResultType } from './types/QueryResultType';

export type None = null | undefined;
export type Nullable<T> = T | None;

export const FILE_RESULT: QueryResultType = 'File';
export const BROWSER_HISTORY_RESULT: QueryResultType = 'BrowserHistory';
export const SCRIPT_RESULT: QueryResultType = 'Script';
export const ACTION_RESULT: QueryResultType = 'Action';
export const CALCULATOR_RESULT: QueryResultType = 'Calculator';
