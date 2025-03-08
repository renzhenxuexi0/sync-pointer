import { generateUuid } from '@/api/util';
import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { locale } from '@tauri-apps/plugin-os';
import { State, store } from 'tauri-plugin-valtio';

const KEY = 'system';

export interface SystemSettings extends State {
  id: string;
  theme: 'light' | 'dark' | 'auto';
  locale: 'zh-CN' | 'en-US' | 'auto';
  autoStart: boolean;
}

const systemSettingsStore = store(
  KEY,
  {
    id: '',
    locale: 'auto',
    theme: 'auto',
    autoStart: false,
  } as SystemSettings,
  {
    saveOnChange: true,
    saveOnExit: true,
    syncStrategy: 'debounce',
    syncInterval: 1000,
  },
);

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
  await systemSettingsStore.start();
  if (systemSettingsStore.state.id === '') {
    systemSettingsStore.state.id = await generateUuid();
  }

  const sys_locale = await detectSystemLocale();
  const locale =
    systemSettingsStore.state.locale === 'auto' ? sys_locale : systemSettingsStore.state.locale;
  const theme = systemSettingsStore.state.theme;
  await i18n.changeLanguage(locale);
  document.documentElement.lang = locale;
  await setTheme(theme === 'auto' ? undefined : theme);
}

async function updateSystemSettings(systemSettings: Partial<SystemSettings>) {
  const locale = systemSettings.locale;
  const theme = systemSettings.theme;
  const autoStart = systemSettings.autoStart;
  if (locale && locale !== systemSettingsStore.state.locale) {
    systemSettingsStore.state.locale = locale;
    if (locale !== 'auto') {
      await i18n.changeLanguage(locale);
      document.documentElement.lang = locale;
    } else {
      const sys_locale = await detectSystemLocale();
      await i18n.changeLanguage(sys_locale);
      document.documentElement.lang = sys_locale;
    }
  }

  if (theme && theme !== systemSettingsStore.state.theme) {
    systemSettingsStore.state.theme = theme;
    await setTheme(theme === 'auto' ? undefined : theme);
  }

  if (autoStart !== undefined && autoStart !== systemSettingsStore.state.autoStart) {
    systemSettingsStore.state.autoStart = autoStart;
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

export { initSystemSettings, systemSettingsStore, updateSystemSettings };
