import { QueryResultType } from './types/QueryResultType';

export type None = null | undefined;
export type Nullable<T> = T | None;

export const FILE_RESULT = 'File' satisfies QueryResultType ;
export const BROWSER_HISTORY_RESULT= 'BrowserHistory' satisfies QueryResultType ;
export const SCRIPT_RESULT= 'Script' satisfies QueryResultType ;
export const ACTION_RESULT = 'Action' satisfies QueryResultType;
export const CALCULATOR_RESULT = 'Calculator' satisfies QueryResultType;
