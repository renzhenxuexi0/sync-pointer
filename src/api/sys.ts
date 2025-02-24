import { invoke } from '@tauri-apps/api/core';

export async function localIp(): Promise<String> {
  return invoke<String>('local_ip');
}
