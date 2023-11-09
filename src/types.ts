import { QueryMode } from './constants';

export type QueryModes = keyof typeof QueryMode;

export type Query = {
search_string: string;
mode: QueryModes;
};

type QueryResultType =
| 'File'
| 'Clipboard'
| 'BrowserHistory'
| 'Script'
| 'Action'
| 'Other';

export interface Result<
Preview,
Kind extends QueryResultType = QueryResultType,
> {
heading: string;
subheading: string;
preview: Preview;
type: Kind;
}

export type FileQueryResult = Result<{ filepath: string }, 'File'>;

export type ClipboardQueryResult = Result<
{ filepath: string } | string,
'Clipboard'
>;

export type BrowserHistoryQueryResult = Result<
{
url: string;
imageUrl: string;
heading: string;
subheading: string;
},
'BrowserHistory'
>;

export type ScriptQueryResult = Result<
{
lang: string;
content: string;
},
'Script'
>;

export type ActionQueryResult = Result<
{
icon: string;
name: string;
description: string;
author: string;
published: string;
},
'Action'
>;

export type QueryResult = {
inline_result: string;
results: QueryResultEntry[];
}

export type OtherQueryResult = Result<null, 'Other'>;

export type QueryResultEntry =
| FileQueryResult
| ClipboardQueryResult
| BrowserHistoryQueryResult
| ScriptQueryResult
| ActionQueryResult;
