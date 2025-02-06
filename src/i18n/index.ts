import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from './locales/en.json';
import zh from './locales/zh.json';

export const defaultNS = 'translation';
export const ns = [defaultNS];

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
    lng: 'zh',
    defaultNS,
    ns,
    fallbackLng: 'zh',
    interpolation: {
      escapeValue: false, // react already safes from xss
    },
  });

export default i18n;
