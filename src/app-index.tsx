import { createRoot } from 'react-dom/client';

import './styles.scss';
import App from './react/ReactApp';
import { useStore } from './react/reactStore';

createRoot(document.getElementById('root')!).render(<App />);
useStore.getState().init();
