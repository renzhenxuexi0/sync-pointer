import { invoke } from '@tauri-apps/api/core';

export async function generateUuid(): Promise<string> {
  return invoke('generate_uuid');
}
