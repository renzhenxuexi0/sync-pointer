import i18n from '@/i18n';
import { invoke } from '@tauri-apps/api/core';
import { theme } from 'antd';
import { store } from 'tauri-plugin-valtio';
const sys_locale: string = await invoke('get_sys_locale');

export interface Preference {
  theme: 'light' | 'dark';
  locale: 'zh' | 'en';
  serverEnabled: boolean;
}

export const preferenceStore = store(
  'preference',
  {
    locale: sys_locale === 'zh-CN' ? 'zh' : 'en',
    theme: 'light',
    serverEnabled: false,
  },
  {
    saveOnChange: true,
    saveInterval: 1000,
  },
);

export function getAntdTheme() {
  if (preferenceStore.state.theme === 'dark') {
    return theme.darkAlgorithm;
  } else if (preferenceStore.state.theme === 'light') {
    return theme.defaultAlgorithm;
  }
}

export function setPreferenceLocale(locale: Preference['locale']) {
  preferenceStore.state.locale = locale;
  i18n.changeLanguage(locale);
  document.documentElement.lang = locale;
}

export function setPreferenceTheme(theme: Preference['theme']) {
  preferenceStore.state.theme = theme;
}
