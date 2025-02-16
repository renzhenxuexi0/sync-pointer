import { LazyStore } from '@tauri-apps/plugin-store';
import { proxy } from 'valtio';

export interface Device {
  hostname: string;
  ip: string;
  port: number;
  id: number;
  serviceType: 'server' | 'client';
}

// 本地 store
const devicesLocalStore = new LazyStore('devices.json');

const devices = await devicesLocalStore.get<Device[]>('preference');

export const preferenceStore = proxy<Device[]>(devices);
