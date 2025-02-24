import { LazyStore } from '@tauri-apps/plugin-store';

const settingsLocalStore = new LazyStore('settings.json');
const devicesLocalStore = new LazyStore('devices.json');

export { devicesLocalStore, settingsLocalStore };

