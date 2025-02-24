import { restartMdns } from '@/api/mdns';
import { localIp } from '@/api/sys';
import { hostname } from '@tauri-apps/plugin-os';
import { proxy, subscribe } from 'valtio';
import { settingsLocalStore } from '..';

const KEY = 'network';

export interface NetworkSettings {
  // 服务类型
  serviceType: 'server' | 'client';
  // 主机名
  hostname: string;
  // IP 地址
  ip: string;
  // 服务发现端口
  discoveryPort: number;
  // 服务端口
  serverPort: number;
  // 是否加密
  encryption: boolean;
  // 加密密码
  encryptionPassword: string;
  // 服务类型
  serverType: 'tcp' | 'udp';
}

// 本地存储
const networkSettingsStore = proxy<NetworkSettings>({
  serviceType: 'client',
  hostname: 'Sync-Pointer',
  discoveryPort: 3456,
  serverPort: 3456,
  encryption: false,
  encryptionPassword: '',
  serverType: 'tcp',
  ip: '',
});

async function initNetworkSettings() {
  const sys_hostname = await hostname();
  const ip = await localIp();
  const settings = await settingsLocalStore.get<NetworkSettings>(KEY);
  const initSettings = {
    hostname: settings?.hostname || sys_hostname || 'Sync-Pointer',
    serviceType: settings?.serviceType || 'client',
    discoveryPort: settings?.discoveryPort || 3456,
    serverPort: settings?.serverPort || 3457,
    encryption: settings?.encryption || false,
    encryptionPassword: settings?.encryptionPassword || '',
    serverType: settings?.serverType || 'udp',
    ip: ip || settings?.ip || '',
  };

  Object.assign(networkSettingsStore, initSettings);
  await settingsLocalStore.set(KEY, initSettings);
  await restartMdns({
    mode: initSettings.serviceType,
    host: initSettings.hostname,
    port: initSettings.discoveryPort,
  });
}

async function updateNetworkSettings(networkSettings: Partial<NetworkSettings>) {
  if (networkSettings.hostname && networkSettings.hostname !== networkSettingsStore.hostname) {
    networkSettingsStore.hostname = networkSettings.hostname;
  }

  if (
    networkSettings.serviceType &&
    networkSettings.serviceType !== networkSettingsStore.serviceType
  ) {
    await restartMdns({
      mode: networkSettings.serviceType,
      host: networkSettingsStore.hostname,
    })
      .then(() => {
        networkSettingsStore.serviceType = networkSettings.serviceType!;
      })
      .catch((e) => {
        console.error(e);
      });
  }
}

// 订阅 store 变化，持久化到本地
subscribe(networkSettingsStore, async () => {
  await settingsLocalStore.set(KEY, networkSettingsStore);
});

export { initNetworkSettings, networkSettingsStore, updateNetworkSettings };

