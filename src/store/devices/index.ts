import { State, store } from 'tauri-plugin-valtio';
import { networkSettingsStore } from '../settings/network';

const KEY = 'devices';

interface Device {
  row: number;
  col: number;
  hostname: string;
  ip: string;
  tcp_port: number;
  mdns_port: number;
  serviceType: 'server' | 'client';
  isMe: boolean;
  status: 'online' | 'offline';
}

export interface DevicesState extends State {
  devices: Record<string, Device>;
  enableCells: Set<number>;
}

// 生成位置key
const createPositionKey = (row: number, col: number) => `${row}-${col}`;
// 解析位置key
const parsePositionKey = (key: string): [number, number] => {
  const [row, col] = key.split('-').map(Number);
  return [row, col];
};

const devicesStore = store(
  KEY,
  {
    devices: {},
    enableCells: new Set<number>(),
  } as DevicesState,
  {
    saveOnChange: true,
    saveOnExit: true,
  },
);

async function initDevices() {
  await devicesStore.start();

  // If devices is empty, initialize with current device
  if (Object.keys(devicesStore.state.devices).length === 0) {
    const positionKey = createPositionKey(2, 2);
    devicesStore.state.devices[positionKey] = {
      row: 2,
      col: 2,
      hostname: networkSettingsStore.state.hostname,
      ip: networkSettingsStore.state.ip,
      tcp_port: networkSettingsStore.state.tcpPort,
      mdns_port: networkSettingsStore.state.mdnsPort,
      serviceType: networkSettingsStore.state.serviceType,
      isMe: true,
      status: 'online',
    };
  }

  initEnableCells();
}

const initEnableCells = () => {
  const enableCells: number[] = [];

  Object.values(devicesStore.state.devices).forEach((device) => {
    const col = device.col;
    const row = device.row;
    enableCells.push(row * 5 + col);
    // 上
    if (row > 0) enableCells.push((row - 1) * 5 + col);
    // 下
    if (row < 4) enableCells.push((row + 1) * 5 + col);
    // 左
    if (col > 0) enableCells.push(row * 5 + col - 1);
    // 右
    if (col < 4) enableCells.push(row * 5 + col + 1);
  });

  // Use Set to remove duplicates
  devicesStore.state.enableCells = new Set(enableCells);
};

const swapDevicePosition = (fromKey: string, toKey: string) => {
  if (fromKey === toKey) return;
  const from = devicesStore.state.devices[fromKey];
  const to = devicesStore.state.devices[toKey];

  if (!to) {
    // 移动到空位置
    const [toRow, toCol] = parsePositionKey(toKey);
    devicesStore.state.devices[toKey] = { ...from, row: toRow, col: toCol };
    delete devicesStore.state.devices[fromKey];
  } else {
    // 替换位置
    devicesStore.state.devices = {
      ...devicesStore.state.devices,
      [fromKey]: { ...to, row: from.row, col: from.col },
      [toKey]: { ...from, row: to.row, col: to.col },
    };
  }

  if (Object.keys(devicesStore.state.devices).length > 1) {
    initEnableCells();
  }
};

export { createPositionKey, devicesStore, initDevices, parsePositionKey, swapDevicePosition };
export type { Device };
