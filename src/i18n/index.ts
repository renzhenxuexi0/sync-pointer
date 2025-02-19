import { Settings } from '@/store/settings';
import { LazyStore } from '@tauri-apps/plugin-store';
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from './locales/en.json';
import zh from './locales/zh.json';

export const defaultNS = 'translation';
export const ns = [defaultNS];

// 本地 store
const settingsLocalStore = new LazyStore('settings.json');
const settings = await settingsLocalStore.get<Settings>('settings');

const resources = {
  en: {
    [defaultNS]: en,
  },
  zh: {
    [defaultNS]: zh,
  },
};

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    resources,
    lng: settings?.systemSettings.locale,
    defaultNS,
    ns,
    fallbackLng: 'zh',
    interpolation: {
      escapeValue: false, // react already safes from xss
    },
  });

export default i18n;
