import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { locale } from '@tauri-apps/plugin-os';
import { LazyStore } from '@tauri-apps/plugin-store';
import { theme } from 'antd';
import { proxy, subscribe } from 'valtio';

// 获取系统语言
const sys_locale = (await locale())?.includes('zh') ? 'zh' : 'en';
// 本地 store
const preferenceLocalStore = new LazyStore('preference.json');
export interface Preference {
  theme: 'light' | 'dark' | 'auto';
  locale: 'zh' | 'en' | 'auto';
  serviceType: 'server' | 'client';
}

const preference = await preferenceLocalStore.get<Preference>('preference');

export const preferenceStore = proxy<Preference>({
  locale: preference?.locale === 'auto' ? sys_locale || 'zh' : preference?.locale || 'auto',
  theme: preference?.theme || 'light',
  serviceType: preference?.serviceType || 'client',
});

export function getAntdTheme() {
  if (preferenceStore.theme === 'dark') {
    return theme.darkAlgorithm;
  } else if (preferenceStore.theme === 'light') {
    return theme.defaultAlgorithm;
  }
}

export function setPreferenceLocale(locale: Preference['locale']) {
  preferenceStore.locale = locale;
  if (locale !== 'auto') {
    i18n.changeLanguage(locale);
    document.documentElement.lang = locale;
  } else {
    i18n.changeLanguage(sys_locale);
    document.documentElement.lang = sys_locale;
  }
}

export function setPreferenceServiceType(serviceType: Preference['serviceType']) {
  preferenceStore.serviceType = serviceType;
}

export async function setPreferenceTheme(theme: Preference['theme']) {
  preferenceStore.theme = theme;
  await setTheme(theme === 'auto' ? undefined : theme);
}
// 订阅 store 变化，持久化到本地
subscribe(preferenceStore, () => {
  preferenceLocalStore.set('preference', preferenceStore);
});
