import { invoke } from '@tauri-apps/api/core';

export async function startMdnsServer(): Promise<void> {
  await invoke<void>('start_mdns_server');
}

export async function stopMdnsServer(): Promise<void> {
  await invoke<void>('stop_mdns_server');
}

export async function startMdnsDiscovery(): Promise<void> {
  await invoke<void>('start_mdns_discovery');
}

export async function stopMdnsDiscovery(): Promise<void> {
  await invoke<void>('stop_mdns_discovery');
}
