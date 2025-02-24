import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { locale } from '@tauri-apps/plugin-os';
import { proxy, subscribe } from 'valtio';
import { settingsLocalStore } from '..';

const KEY = 'system';

export interface SystemSettings {
  theme: 'light' | 'dark' | 'auto';
  locale: 'zh-CN' | 'en-US' | 'auto';
  autoStart: boolean;
}

const systemSettingsStore = proxy<SystemSettings>({
  locale: 'auto',
  theme: 'light',
  autoStart: false,
});

async function detectSystemLocale(): Promise<'zh-CN' | 'en-US'> {
  try {
    const localeStr = (await locale()) || '';
    return String(localeStr).toLowerCase().includes('zh') ? 'zh-CN' : 'en-US';
  } catch (error) {
    console.error('Failed to detect system locale:', error);
    return 'en-US';
  }
}

async function initSystemSettings() {
  const sys_locale = await detectSystemLocale();
  const settings = await settingsLocalStore.get<SystemSettings>(KEY);

  const initSettings = {
    locale: settings?.locale === 'auto' ? sys_locale : settings?.locale || 'auto',
    theme: settings?.theme || 'light',
    autoStart: settings?.autoStart || false,
  } as SystemSettings;

  Object.assign(systemSettingsStore, initSettings);
  i18n.changeLanguage(initSettings.locale);
  document.documentElement.lang = initSettings.locale;
  await setTheme(initSettings.theme === 'auto' ? undefined : initSettings.theme);
  await settingsLocalStore.set('settings', initSettings);
}

async function updateSystemSettings(systemSettings: Partial<SystemSettings>) {
  const locale = systemSettings.locale;
  const theme = systemSettings.theme;
  const autoStart = systemSettings.autoStart;
  console.log(systemSettings);
  if (locale && locale !== systemSettingsStore.locale) {
    systemSettingsStore.locale = locale;
    if (locale !== 'auto') {
      await i18n.changeLanguage(locale);
      document.documentElement.lang = locale;
    } else {
      const sys_locale = await detectSystemLocale();
      await i18n.changeLanguage(sys_locale);
      document.documentElement.lang = sys_locale;
    }
  }

  if (theme && theme !== systemSettingsStore.theme) {
    systemSettingsStore.theme = theme;
    await setTheme(theme === 'auto' ? undefined : theme);
  }

  if (autoStart !== undefined && autoStart !== systemSettingsStore.autoStart) {
    systemSettingsStore.autoStart = autoStart;
    if (autoStart) {
      await enable();
    } else {
      await isEnabled().then(async (enabled) => {
        if (enabled) {
          await disable();
        }
      });
    }
  }
}

// 订阅 store 变化，持久化到本地
subscribe(systemSettingsStore, async () => {
  await settingsLocalStore.set(KEY, systemSettingsStore);
});

export { initSystemSettings, systemSettingsStore, updateSystemSettings };

