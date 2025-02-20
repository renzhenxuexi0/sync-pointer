import { invoke } from '@tauri-apps/api/core';

export async function trace(message: string): Promise<void> {
  await invoke<void>('trace', { message });
}

export async function debug(message: string): Promise<void> {
  await invoke<void>('debug', { message });
}

export async function info(message: string): Promise<void> {
  await invoke<void>('info', { message });
}

export async function warn(message: string): Promise<void> {
  await invoke<void>('warn', { message });
}

export async function error(message: string): Promise<void> {
  await invoke<void>('error', { message });
}
