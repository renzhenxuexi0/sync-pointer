import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import enUS from './locales/en-US.json';
import zhCN from './locales/zh-CN.json';

export const defaultNS = 'translation';
export const ns = [defaultNS];

const resources = {
  'en-US': {
    [defaultNS]: enUS,
  },
  'zh-CN': {
    [defaultNS]: zhCN,
  },
};

// 初始化一个基础的 i18n 实例，后续会通过 initializeI18n 更新配置
i18n.use(initReactI18next).init({
  resources,
  lng: 'zh-CN',
  defaultNS,
  ns,
  fallbackLng: 'zh-CN',
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
