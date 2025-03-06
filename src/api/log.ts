import { invoke } from '@tauri-apps/api/core';

export async function trace(message: string): Promise<void> {
  await invoke('trace', { message });
}

export async function debug(message: string): Promise<void> {
  await invoke('debug', { message });
}

export async function info(message: string): Promise<void> {
  await invoke('info', { message });
}

export async function warn(message: string): Promise<void> {
  await invoke('warn', { message });
}

export async function error(message: string): Promise<void> {
  await invoke('error', { message });
}
