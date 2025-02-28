import { invoke } from '@tauri-apps/api/core';

/** Start the MDNS client */
export async function startMdnsClient(): Promise<void> {
  return invoke<void>('start_mdns_client');
}

/** Stop the MDNS client */
export async function stopMdnsClient(): Promise<void> {
  return invoke<void>('stop_mdns_client');
}

/** Start the MDNS server */
export async function startMdnsServer(): Promise<void> {
  return invoke<void>('start_mdns_server');
}

/** Stop the MDNS server */
export async function stopMdnsServer(): Promise<void> {
  return invoke<void>('stop_mdns_server');
}

export interface UpdateServerInfoOptions {
  host?: string;
  mdnsPort?: number;
  tcpPort?: number;
}

/** Update the MDNS server configuration */
export async function updateMdnsServerInfo(options: UpdateServerInfoOptions = {}): Promise<void> {
  return invoke<void>('update_mdns_server_info', {
    host: options.host,
    mdns_port: options.mdnsPort,
    tcp_port: options.tcpPort,
  });
}
