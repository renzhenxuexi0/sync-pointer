import { invoke } from '@tauri-apps/api/core';

/**
 * 启动服务（基于当前服务类型）
 * @returns Promise<void>
 */
export async function startService(serviceType: 'client' | 'server'): Promise<void> {
  return invoke('start_service', { serviceType });
}

/**
 * 切换服务类型并重启服务
 * @param serviceType 'server' | 'client'
 * @returns Promise<void>
 */
export async function handleServiceTypeChange(serviceType: 'server' | 'client'): Promise<void> {
  return invoke('handle_service_type_change', { serviceType });
}

export interface UpdateServerInfoOptions {
  host?: string;
  mdnsPort?: number;
  tcpPort?: number;
}

/**
 * 重启服务（基于当前服务类型）
 * @returns Promise<void>
 */
export async function restartService(serviceType: 'client' | 'server'): Promise<void> {
  return invoke('restart_service', { serviceType });
}
