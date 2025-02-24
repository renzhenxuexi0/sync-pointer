import { invoke } from '@tauri-apps/api/core';

export interface MdnsRestartOptions {
  mode: 'server' | 'client';
  host?: string;
  port?: number;
}

export async function restartMdns(options: MdnsRestartOptions): Promise<void> {
  return invoke<void>('restart_mdns', { ...options });
}
