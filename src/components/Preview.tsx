import { Match, Switch, useContext } from 'solid-js';
import { StoreContext } from '../store';

export default function Preview() {
  const [state] = useContext(StoreContext);

  return (
    <div class="preview-container">
      <Switch fallback={<div>No Preview</div>}>
        <Match
          when={
            state.queryResult.results[state.cursor]?.type === 'BrowserHistory'
          }
        >
          <iframe
            width="300"
            height="200"
            sandbox=""
            src={state.queryResult.results[state.cursor].subheading}
            style={{ width: '100%', height: '100%' }}
          />
        </Match>
      </Switch>
    </div>
  );
}
