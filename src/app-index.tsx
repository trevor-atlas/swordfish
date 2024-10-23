import { createRoot } from 'react-dom/client';

import './styles.scss';
import App from './react/ReactApp';
import { useStore } from './react/reactStore';

useStore.getState().init();
createRoot(document.getElementById('root')!).render(<App />);
