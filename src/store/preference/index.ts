import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { hostname, locale } from '@tauri-apps/plugin-os';
import { LazyStore } from '@tauri-apps/plugin-store';
import { proxy, subscribe } from 'valtio';

// 获取系统语言
const sys_locale = (await locale())?.includes('zh') ? 'zh' : 'en';
const sys_hostname = await hostname();
// 本地 store
const preferenceLocalStore = new LazyStore('preference.json');
export interface Preference {
  systemSettings: {
    theme: 'light' | 'dark' | 'auto';
    locale: 'zh' | 'en' | 'auto';
  };
  serviceSettings: {
    serviceType: 'server' | 'client';
    hostname: string;
  };
}

const preference = await preferenceLocalStore.get<Preference>('preference');

export const preferenceStore = proxy<Preference>({
  systemSettings: {
    locale:
      preference?.systemSettings?.locale === 'auto'
        ? sys_locale || 'zh'
        : preference?.systemSettings?.locale || 'auto',
    theme: preference?.systemSettings?.theme || 'light',
  },
  serviceSettings: {
    hostname: preference?.serviceSettings?.hostname || sys_hostname || 'Sync-Pointer',
    serviceType: preference?.serviceSettings?.serviceType || 'client',
  },
});

export function updateSystemSettings(systemSettings: Preference['systemSettings']) {
  const locale = systemSettings.locale;
  const theme = systemSettings.theme;
  if (locale) {
    preferenceStore.systemSettings.locale = locale;
    if (locale !== 'auto') {
      i18n.changeLanguage(locale);
      document.documentElement.lang = locale;
    } else {
      i18n.changeLanguage(sys_locale);
      document.documentElement.lang = sys_locale;
    }
  }

  if (theme) {
    preferenceStore.systemSettings.theme = theme;
    setTheme(theme === 'auto' ? undefined : theme);
  }
}

export function updateServiceSettings(serviceSettings: Preference['serviceSettings']) {
  if (serviceSettings.hostname === '') {
    serviceSettings.hostname = sys_hostname || 'Sync-Pointer';
  }
  preferenceStore.serviceSettings = serviceSettings;
}

// 订阅 store 变化，持久化到本地
subscribe(preferenceStore, () => {
  preferenceLocalStore.set('preference', preferenceStore);
});
