import '@ant-design/v5-patch-for-react-19';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import App from './App';
import { initDevices } from './store/devices';
import { initNetworkSettings } from './store/settings/network';
import { initSystemSettings } from './store/settings/system';

// 初始化应用
async function initializeApp() {
  try {
    // 按顺序初始化
    await initSystemSettings();
    await initNetworkSettings();
    await initDevices();

    // 初始化完成后渲染应用
    ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
      <BrowserRouter>
        <App />
      </BrowserRouter>,
    );
  } catch (error) {
    console.error('Failed to initialize app:', error);
  }
}

// 启动应用
initializeApp();
