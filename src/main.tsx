import '@ant-design/v5-patch-for-react-19';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import App from './App';
import './i18n';
import { initStore } from './store';
import './tauri/tray';

await initStore();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <BrowserRouter>
    <App />
  </BrowserRouter>,
);
