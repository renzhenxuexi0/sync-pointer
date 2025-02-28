import {
  startMdnsClient,
  startMdnsServer,
  stopMdnsClient,
  stopMdnsServer,
  updateMdnsServerInfo,
} from '@/api/mdns';
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
  // tcp端口 用于监听客户端连接和维护会话数据传输
  tcpPort: number;
}

// 本地存储
const networkSettingsStore = proxy<NetworkSettings>({
  serviceType: 'client',
  hostname: 'Sync-Pointer',
  mdnsPort: 3456,
  tcpPort: 3457,
  // 不可修改
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
    ip: ip || settings?.ip || '',
  };

  Object.assign(networkSettingsStore, initSettings);
  await settingsLocalStore.set(KEY, initSettings);
  await updateMdnsServerInfo({
    host: networkSettingsStore.hostname,
    mdnsPort: networkSettingsStore.mdnsPort,
    tcpPort: networkSettingsStore.tcpPort,
  });
  if (networkSettingsStore.serviceType === 'server') {
    await startMdnsServer();
  } else {
    await startMdnsClient();
  }
}

async function updateNetworkSettings(networkSettings: Partial<NetworkSettings>) {
  let isNeedUpdate = false;
  if (networkSettings.hostname && networkSettings.hostname !== networkSettingsStore.hostname) {
    networkSettingsStore.hostname = networkSettings.hostname;
    isNeedUpdate = true;
  }

  if (networkSettings.mdnsPort && networkSettings.mdnsPort !== networkSettingsStore.mdnsPort) {
    networkSettingsStore.mdnsPort = networkSettings.mdnsPort;
    isNeedUpdate = true;
  }

  if (networkSettings.tcpPort && networkSettings.tcpPort !== networkSettingsStore.tcpPort) {
    networkSettingsStore.tcpPort = networkSettings.tcpPort;
    isNeedUpdate = true;
  }

  if (isNeedUpdate) {
    updateMdnsServerInfo({
      host: networkSettings.hostname,
      mdnsPort: networkSettings.mdnsPort,
      tcpPort: networkSettings.tcpPort,
    });
  }

  if (
    networkSettings.serviceType &&
    networkSettings.serviceType !== networkSettingsStore.serviceType
  ) {
    if (networkSettings.serviceType === 'server') {
      // 关闭客户端
      await stopMdnsClient();
      // 开启服务端
      await startMdnsServer();
    } else {
      // 关闭服务端
      await stopMdnsServer();
      // 开启客户端
      await startMdnsClient();
    }
  }
}

// 订阅 store 变化，持久化到本地
subscribe(networkSettingsStore, async () => {
  await settingsLocalStore.set(KEY, networkSettingsStore);
});

export { initNetworkSettings, networkSettingsStore, updateNetworkSettings };
