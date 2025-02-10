import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { debug } from '@tauri-apps/plugin-log';
import { theme } from 'antd';
import { store } from 'tauri-plugin-valtio';

export interface Preference {
  theme: 'light' | 'dark';
  locale: 'zh' | 'en';
  serverEnabled: boolean;
}

export const preferenceStore = store(
  'preference',
  {
    locale: 'zh',
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
  debug(`切换语言 ${locale}`);
}

export function setPreferenceTheme(theme: Preference['theme']) {
  preferenceStore.state.theme = theme;
  setTheme(theme);
  debug(`切换主题 ${theme}`);
}
