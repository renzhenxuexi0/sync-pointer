import { setTheme } from '@tauri-apps/api/app';
import { theme } from 'antd';
import { store } from 'tauri-plugin-valtio';

export interface Preference {
  theme: 'light' | 'dark';
  locale: 'zh' | 'en';
}

export const preferenceStore = store('preference', {
  locale: 'zh',
  theme: 'light',
});

export function getAntdTheme() {
  if (preferenceStore.state.theme === 'dark') {
    return theme.darkAlgorithm;
  } else if (preferenceStore.state.theme === 'light') {
    return theme.defaultAlgorithm;
  }
}

export function setPreferenceLocale(locale: Preference['locale']) {
  preferenceStore.state.locale = locale;
}

export function setPreferenceTheme(theme: Preference['theme']) {
  preferenceStore.state.theme = theme;
  setTheme(theme);
}
