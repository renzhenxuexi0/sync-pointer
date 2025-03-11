import { handleServiceTypeChange, restartService, startService } from '@/api/service';
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
  // 启动服务
  await startService(networkSettingsStore.state.serviceType);
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

  if (
    networkSettings.serviceType &&
    networkSettings.serviceType !== networkSettingsStore.state.serviceType
  ) {
    networkSettingsStore.state.serviceType = networkSettings.serviceType;
    await handleServiceTypeChange(networkSettingsStore.state.serviceType);
  } else if (isNeedUpdate) {
    await restartService(networkSettingsStore.state.serviceType);
  }
}

export { initNetworkSettings, networkSettingsStore, updateNetworkSettings };
