import { LazyStore } from '@tauri-apps/plugin-store';
import { subscribe } from 'valtio';
import { proxyMap, proxySet } from 'valtio/utils';
import { settingsStore } from '../settings';

interface Device {
  row: number;
  col: number;
  hostname: string;
  ip: string;
  port: number;
  serviceType: 'server' | 'client';
  isMe: boolean;
  status: 'online' | 'offline';
}

// 生成位置key
const createPositionKey = (row: number, col: number) => `${row}-${col}`;
// 解析位置key
const parsePositionKey = (key: string): [number, number] => {
  const [row, col] = key.split('-').map(Number);
  return [row, col];
};

// 本地 store
const devicesLocalStore = new LazyStore('devices.json');

// 初始状态
const initialDevicesMap = new Map<string, Device>();
const defaultDevice: Device = {
  row: 2,
  col: 2,
  hostname: settingsStore.serviceSettings.hostname,
  ip: '',
  port: 0,
  serviceType: 'server' as const,
  isMe: true,
  status: 'online',
};
initialDevicesMap.set(createPositionKey(defaultDevice.row, defaultDevice.col), defaultDevice);

const devicesStore = proxyMap(initialDevicesMap);
const enableCellsStore = proxySet(new Set<number>());

async function initializeDevices() {
  const devicesArray = await devicesLocalStore.get<Device[]>('devices');

  if (devicesArray) {
    devicesStore.clear();
    devicesArray.forEach((device) => {
      devicesStore.set(createPositionKey(device.row, device.col), device);
    });
  }

  initEnableCells();
}

const initEnableCells = () => {
  enableCellsStore.clear();
  devicesStore.forEach((device) => {
    const col = device.col;
    const row = device.row;
    enableCellsStore.add(row * 5 + col);
    // 上
    if (row > 0) enableCellsStore.add((row - 1) * 5 + col);
    // 下
    if (row < 4) enableCellsStore.add((row + 1) * 5 + col);
    // 左
    if (col > 0) enableCellsStore.add(row * 5 + col - 1);
    // 右
    if (col < 4) enableCellsStore.add(row * 5 + col + 1);
  });
};

initEnableCells();

const swapDevicePosition = (fromKey: string, toKey: string) => {
  if (fromKey === toKey) return;
  const from = devicesStore.get(fromKey);
  const to = devicesStore.get(toKey);

  if (!from) return;

  if (!to) {
    // 移动到空位置
    const [toRow, toCol] = parsePositionKey(toKey);
    devicesStore.delete(fromKey);
    devicesStore.set(toKey, { ...from, row: toRow, col: toCol });
  } else {
    // 替换位置
    devicesStore.set(fromKey, { ...to, row: from.row, col: from.col });
    devicesStore.set(toKey, { ...from, row: to.row, col: to.col });
  }
};

// 订阅 store 变化，持久化到本地
subscribe(devicesStore, () => {
  devicesLocalStore.set('devices', Array.from(devicesStore.values()));
  initEnableCells();
});

export {
  createPositionKey,
  devicesStore,
  enableCellsStore,
  initializeDevices,
  parsePositionKey,
  swapDevicePosition
};
export type { Device };


