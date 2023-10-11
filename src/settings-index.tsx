/* @refresh reload */
import { render } from 'solid-js/web';

import './styles.scss';
import App from './App';
import Settings from './Settings';

render(() => <Settings />, document.getElementById('root') as HTMLElement);
