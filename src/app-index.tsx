/* @refresh reload */
import { render } from 'solid-js/web';

import './styles.scss';
import App from './App';
import { StoreProvider } from './store';

render(
  () => (
    <StoreProvider>
      <App />
    </StoreProvider>
  ),
  document.getElementById('root') as HTMLElement,
);
