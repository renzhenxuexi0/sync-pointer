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

// 初始化一个基础的 i18n 实例，后续会通过 initializeI18n 更新配置
i18n.use(initReactI18next).init({
  resources,
  lng: 'zh',
  defaultNS,
  ns,
  fallbackLng: 'zh',
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
