import { Match, Switch, useContext } from 'solid-js';
import { StoreContext } from '../store';

export default function Preview() {
  const [state] = useContext(StoreContext);

  return (
    <div class="preview-container">
      <Switch fallback={<div>No Preview</div>}>
        <Match when={state.queryResult[state.cursor]?.preview?.text}>
          {state.queryResult[state.cursor].preview.text}
        </Match>
      </Switch>
    </div>
  );
}
