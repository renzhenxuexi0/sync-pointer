import {
  startMdnsClient,
  startMdnsServer,
  stopMdnsClient,
  stopMdnsServer,
  updateMdnsServerInfo,
} from '@/api/mdns';
import { localIp } from '@/api/sys';
import { hostname } from '@tauri-apps/plugin-os';
import { State, store } from 'tauri-plugin-valtio';

const KEY = 'network';

export interface NetworkSettings extends State {
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

const networkSettingsStore = store(
  KEY,
  {
    serviceType: 'client',
    hostname: 'Sync-Pointer',
    mdnsPort: 3456,
    tcpPort: 3457,
    ip: '',
  } as NetworkSettings,
  {
    saveOnChange: true,
    saveOnExit: true,
  },
);

async function initNetworkSettings() {
  await networkSettingsStore.start();
  const sys_hostname = await hostname();
  const ip = await localIp();

  if (!networkSettingsStore.state.hostname) {
    networkSettingsStore.state.hostname = sys_hostname || 'Sync-Pointer';
  }

  networkSettingsStore.state.ip = ip || networkSettingsStore.state.ip || '';

  await updateMdnsServerInfo({
    host: networkSettingsStore.state.hostname,
    mdnsPort: networkSettingsStore.state.mdnsPort,
    tcpPort: networkSettingsStore.state.tcpPort,
  });

  if (networkSettingsStore.state.serviceType === 'server') {
    await startMdnsServer();
  } else {
    await startMdnsClient();
  }
}

async function updateNetworkSettings(networkSettings: Partial<NetworkSettings>) {
  let isNeedUpdate = false;

  if (
    networkSettings.hostname &&
    networkSettings.hostname !== networkSettingsStore.state.hostname
  ) {
    networkSettingsStore.state.hostname = networkSettings.hostname;
    isNeedUpdate = true;
  }

  if (
    networkSettings.mdnsPort &&
    networkSettings.mdnsPort !== networkSettingsStore.state.mdnsPort
  ) {
    networkSettingsStore.state.mdnsPort = networkSettings.mdnsPort;
    isNeedUpdate = true;
  }

  if (networkSettings.tcpPort && networkSettings.tcpPort !== networkSettingsStore.state.tcpPort) {
    networkSettingsStore.state.tcpPort = networkSettings.tcpPort;
    isNeedUpdate = true;
  }

  if (isNeedUpdate) {
    updateMdnsServerInfo({
      host: networkSettingsStore.state.hostname,
      mdnsPort: networkSettingsStore.state.mdnsPort,
      tcpPort: networkSettingsStore.state.tcpPort,
    });
  }

  if (
    networkSettings.serviceType &&
    networkSettings.serviceType !== networkSettingsStore.state.serviceType
  ) {
    networkSettingsStore.state.serviceType = networkSettings.serviceType;

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

export { initNetworkSettings, networkSettingsStore, updateNetworkSettings };
