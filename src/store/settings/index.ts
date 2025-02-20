import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { hostname, locale } from '@tauri-apps/plugin-os';
import { LazyStore } from '@tauri-apps/plugin-store';
import { proxy, subscribe } from 'valtio';

export interface Settings {
  systemSettings: {
    theme: 'light' | 'dark' | 'auto';
    locale: 'zh' | 'en' | 'auto';
    autoStart: boolean;
  };
  serviceSettings: {
    serviceType: 'server' | 'client';
    hostname: string;
  };
  serverSettings: {
    tcpPort: number;
    mdnsPort: number;
  };
  clientSettings: {
    connectServer: string;
  };
}

// 获取系统语言
const sys_locale = (await locale())?.includes('zh') ? 'zh' : 'en';
const sys_hostname = await hostname();
// 本地 store
const settingsLocalStore = new LazyStore('settings.json');
const settings = await settingsLocalStore.get<Settings>('settings');

const initSettings = {
  systemSettings: {
    locale:
      settings?.systemSettings?.locale === 'auto'
        ? sys_locale || 'zh'
        : settings?.systemSettings?.locale || 'auto',
    theme: settings?.systemSettings?.theme || 'light',
    autoStart: settings?.systemSettings?.autoStart || false,
  },
  serviceSettings: {
    hostname: settings?.serviceSettings?.hostname || sys_hostname || 'Sync-Pointer',
    serviceType: settings?.serviceSettings?.serviceType || 'client',
  },
} as Settings;
// 初始化 store
await settingsLocalStore.set('settings', initSettings);

export const settingsStore = proxy<Settings>(initSettings);

export async function updateSystemSettings(systemSettings: Partial<Settings['systemSettings']>) {
  const locale = systemSettings.locale;
  const theme = systemSettings.theme;
  const autoStart = systemSettings.autoStart;
  if (locale) {
    settingsStore.systemSettings.locale = locale;
    if (locale !== 'auto') {
      await i18n.changeLanguage(locale);
      document.documentElement.lang = locale;
    } else {
      await i18n.changeLanguage(sys_locale);
      document.documentElement.lang = sys_locale;
    }
  }

  if (theme) {
    settingsStore.systemSettings.theme = theme;
    await setTheme(theme === 'auto' ? undefined : theme);
  }

  if (autoStart !== undefined) {
    settingsStore.systemSettings.autoStart = autoStart;
    if (autoStart) {
      await enable();
    } else {
      await isEnabled().then((enabled) => {
        if (enabled) {
          disable();
        }
      });
    }
  }
}

export function updateServiceSettings(serviceSettings: Partial<Settings['serviceSettings']>) {
  if (serviceSettings.hostname === '' || serviceSettings.hostname) {
    serviceSettings.hostname = sys_hostname || 'Sync-Pointer';
  }
}

// 订阅 store 变化，持久化到本地
subscribe(settingsStore, async () => {
  await settingsLocalStore.set('settings', settingsStore);
});
