import { QueryMode } from './constants';

export type None = null | undefined;
export type Nullable<T> = T | None;
export const isSome = <T>(a: Nullable<T>): a is T => typeof a != null;
export const isNone = <T>(a: Nullable<T>): a is None => !isSome(a);

export type Query = {
  search_string: string;
  mode: QueryMode;
};

export const FILE_RESULT = 'File';
export const BROWSER_HISTORY_RESULT = 'BrowserHistory';
export const SCRIPT_RESULT = 'Script';
export const ACTION_RESULT = 'Action';
export const CALCULATOR_RESULT = 'Calculator';

type QueryResultType =
  | typeof FILE_RESULT
  | typeof BROWSER_HISTORY_RESULT
  | typeof SCRIPT_RESULT
  | typeof ACTION_RESULT
  | typeof CALCULATOR_RESULT;

export interface QueryResult<Preview> {
  iconPath: string;
  heading: string;
  subheading: string;
  value: string;
  preview: Preview;
  type: QueryResultType;
}

export interface FileQueryResult
  extends QueryResult<{
    iconPath: string;
    path: string;
    filename: string;
    extension: string;
    size: string;
    last_modified: string;
    content: string;
    parsed_content: string;
  }> {
  type: typeof FILE_RESULT;
}

export interface BrowserHistoryQueryResult
  extends QueryResult<{
    url: string;
    imageUrl: string;
    iconPath: Nullable<string>;
  }> {
  type: typeof BROWSER_HISTORY_RESULT;
}

export interface ScriptQueryResult
  extends QueryResult<{
    heading: string;
    subheading: string;
    content: string;
    language: string;
    parsedContent: Nullable<string>;
  }> {
  type: typeof SCRIPT_RESULT;
}

// workflow? this is a bit different from the others
// and could become something like alfred workflows
export interface ActionQueryResult
  extends QueryResult<{
    iconPath: string;
    name: string;
    description: string;
    author: string;
    published: string;
  }> {
  type: typeof ACTION_RESULT;
}

export interface CalculatorQueryResult
  extends QueryResult<{ parsedContent: string }> {
  type: typeof CALCULATOR_RESULT;
}

export type QueryResultEntry =
  | FileQueryResult
  | BrowserHistoryQueryResult
  | ScriptQueryResult
  | ActionQueryResult
  | CalculatorQueryResult;

export type QueryResponse = {
  results: QueryResultEntry[];
};
