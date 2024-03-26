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
export const CLIPBOARD_RESULT = 'Clipboard';
export const BROWSER_HISTORY_RESULT = 'BrowserHistory';
export const SCRIPT_RESULT = 'Script';
export const ACTION_RESULT = 'Action';
export const OTHER_RESULT = 'Other';

type QueryResultType =
  | typeof FILE_RESULT
  | typeof CLIPBOARD_RESULT
  | typeof BROWSER_HISTORY_RESULT
  | typeof SCRIPT_RESULT
  | typeof ACTION_RESULT
  | typeof OTHER_RESULT;

export interface Result<
  Preview,
  Kind extends QueryResultType = QueryResultType,
> {
  heading: string;
  subheading: string;
  preview: Preview;
  type: Kind;
}

export type FileQueryResult = Result<{ filepath: string }, typeof FILE_RESULT>;

export type ClipboardQueryResult = Result<
  { filepath: string } | string,
  typeof CLIPBOARD_RESULT
>;

export type BrowserHistoryQueryResult = Result<
  {
    url: string;
    imageUrl: string;
    heading: string;
    subheading: string;
  },
  typeof BROWSER_HISTORY_RESULT
>;

export type ScriptQueryResult = Result<
  {
    lang: string;
    content: string;
  },
  typeof SCRIPT_RESULT
>;

export type ActionQueryResult = Result<
  {
    icon: string;
    name: string;
    description: string;
    author: string;
    published: string;
  },
  typeof ACTION_RESULT
>;

export type QueryResult = {
  results: QueryResultEntry[];
};

export type OtherQueryResult = Result<null, typeof OTHER_RESULT>;

export type QueryResultEntry =
  | FileQueryResult
  | ClipboardQueryResult
  | BrowserHistoryQueryResult
  | ScriptQueryResult
  | ActionQueryResult;
