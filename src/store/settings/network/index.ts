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
  // 用于服务发现端口
  mdnsPort: number;
  // tcp端口 用于监听客户端连接和维护会话
  tcpPort: number;
  // udp端口 用于数据传输
  udpPort: number;
}

// 本地存储
const networkSettingsStore = proxy<NetworkSettings>({
  serviceType: 'client',
  hostname: 'Sync-Pointer',
  mdnsPort: 3456,
  tcpPort: 3457,
  udpPort: 3458,
  ip: '',
});

async function initNetworkSettings() {
  const sys_hostname = await hostname();
  const ip = await localIp();
  const settings = await settingsLocalStore.get<NetworkSettings>(KEY);
  const initSettings = {
    hostname: settings?.hostname || sys_hostname || 'Sync-Pointer',
    serviceType: settings?.serviceType || 'client',
    discoveryPort: settings?.mdnsPort || 3456,
    tcpPort: settings?.tcpPort || 3457,
    udpPort: settings?.udpPort || 3458,
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

