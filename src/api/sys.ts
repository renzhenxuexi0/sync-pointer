import { invoke } from '@tauri-apps/api/core';

export async function localIp(): Promise<string> {
  return invoke<string>('local_ip');
}
