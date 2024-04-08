import { Match, Show, Switch, useContext } from 'solid-js';
import { StoreContext } from '../store';
import {
  BROWSER_HISTORY_RESULT,
  CALCULATOR_RESULT,
  FILE_RESULT,
  FileQueryResult,
} from '../types';

export default function Preview() {
  const [state, { getSelectedResult }] = useContext(StoreContext);
  const result = () => getSelectedResult();

  return (
    <div class="preview-container">
      <Switch fallback={null}>
        <Match when={result()?.type === BROWSER_HISTORY_RESULT}>
          {/* <iframe
            width="300"
            height="200"
            sandbox=""
            src={state.queryResult.results[state.cursor].subheading}
            style={{ width: '100%', height: '100%' }}
          /> */}
        </Match>
        <Match when={result()?.type === FILE_RESULT}>
          <div class="flex flex-col">
            <div>{(result() as FileQueryResult).preview.filepath}</div>
            <div>
              {JSON.stringify((result() as FileQueryResult).preview, null, 2)}
            </div>
          </div>
        </Match>
        <Match when={result()?.type === CALCULATOR_RESULT}>
          <div innerHTML={result()?.preview?.parsedContent} />
        </Match>
      </Switch>
    </div>
  );
}
