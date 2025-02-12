import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { invoke } from '@tauri-apps/api/core';
import { LazyStore } from '@tauri-apps/plugin-store';
import { theme } from 'antd';
import { proxy, subscribe } from 'valtio';
const sys_locale: string = await invoke('get_sys_locale');

// 本地 store
const preferenceLocalStore = new LazyStore('preference.json');

export interface Preference {
  theme: 'light' | 'dark';
  locale: 'zh' | 'en';
  serverEnabled: boolean;
  serverStarted: boolean;
}

const preference = await preferenceLocalStore.get<Preference>('preference');

export const preferenceStore = proxy<Preference>({
  locale: preference?.locale || (sys_locale === 'zh-CN' ? 'zh' : 'en'),
  theme: preference?.theme || 'light',
  serverEnabled: preference?.serverEnabled || false,
  serverStarted: preference?.serverStarted || false,
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
  i18n.changeLanguage(locale);
  document.documentElement.lang = locale;
}

export async function setPreferenceTheme(theme: Preference['theme']) {
  preferenceStore.theme = theme;
  await setTheme(theme);
}
subscribe(preferenceStore, () => {
  preferenceLocalStore.set('preference', preferenceStore);
});
