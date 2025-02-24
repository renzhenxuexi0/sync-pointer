import { restartMdns } from '@/api/mdns';
import i18n from '@/i18n';
import { setTheme } from '@tauri-apps/api/app';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { hostname, locale } from '@tauri-apps/plugin-os';
import { LazyStore } from '@tauri-apps/plugin-store';
import { proxy, subscribe } from 'valtio';

export interface Settings {
  systemSettings: {
    theme: 'light' | 'dark' | 'auto';
    locale: 'zh-CN' | 'en-US' | 'auto';
    autoStart: boolean;
  };
  serviceSettings: {
    serviceType: 'server' | 'client';
    hostname: string;
  };
  serverSettings: {
    tcpPort: number;
    mdnsPort: number;
  };
  clientSettings: {
    connectServer: string;
  };
}

// 本地存储
const settingsLocalStore = new LazyStore('settings.json');
//
const settingsStore = proxy<Settings>({
  systemSettings: {
    locale: 'auto',
    theme: 'light',
    autoStart: false,
  },
  serviceSettings: {
    hostname: 'Sync-Pointer',
    serviceType: 'client',
  },
  serverSettings: {
    tcpPort: 0,
    mdnsPort: 0,
  },
  clientSettings: {
    connectServer: '',
  },
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

async function initializeSettings() {
  const sys_locale = await detectSystemLocale();
  const sys_hostname = await hostname();
  const settings = await settingsLocalStore.get<Settings>('settings');

  const initSettings = {
    systemSettings: {
      locale:
        settings?.systemSettings?.locale === 'auto'
          ? sys_locale
          : settings?.systemSettings?.locale || 'auto',
      theme: settings?.systemSettings?.theme || 'light',
      autoStart: settings?.systemSettings?.autoStart || false,
    },
    serviceSettings: {
      hostname: settings?.serviceSettings?.hostname || sys_hostname || 'Sync-Pointer',
      serviceType: settings?.serviceSettings?.serviceType || 'client',
    },
    serverSettings: settings?.serverSettings || {
      tcpPort: 0,
      mdnsPort: 0,
    },
    clientSettings: settings?.clientSettings || {
      connectServer: '',
    },
  } as Settings;

  Object.assign(settingsStore, initSettings);
  i18n.changeLanguage(initSettings.systemSettings.locale);
  document.documentElement.lang = initSettings.systemSettings.locale;
  await setTheme(
    initSettings.systemSettings.theme === 'auto' ? undefined : initSettings.systemSettings.theme,
  );
  await settingsLocalStore.set('settings', initSettings);
  await restartMdns({
    mode: initSettings.serviceSettings.serviceType,
    host: initSettings.serviceSettings.hostname,
  });

  return sys_locale;
}

async function updateSystemSettings(systemSettings: Partial<Settings['systemSettings']>) {
  const locale = systemSettings.locale;
  const theme = systemSettings.theme;
  const autoStart = systemSettings.autoStart;
  console.log(systemSettings);
  if (locale && locale !== settingsStore.systemSettings.locale) {
    settingsStore.systemSettings.locale = locale;
    if (locale !== 'auto') {
      await i18n.changeLanguage(locale);
      document.documentElement.lang = locale;
    } else {
      const sys_locale = await detectSystemLocale();
      await i18n.changeLanguage(sys_locale);
      document.documentElement.lang = sys_locale;
    }
  }

  if (theme && theme !== settingsStore.systemSettings.theme) {
    settingsStore.systemSettings.theme = theme;
    await setTheme(theme === 'auto' ? undefined : theme);
  }

  if (autoStart !== undefined && autoStart !== settingsStore.systemSettings.autoStart) {
    settingsStore.systemSettings.autoStart = autoStart;
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

async function updateServiceSettings(serviceSettings: Partial<Settings['serviceSettings']>) {
  console.log(serviceSettings);
  if (
    serviceSettings.hostname &&
    serviceSettings.hostname !== settingsStore.serviceSettings.hostname
  ) {
    settingsStore.serviceSettings.hostname = serviceSettings.hostname;
  }

  if (
    serviceSettings.serviceType &&
    serviceSettings.serviceType !== settingsStore.serviceSettings.serviceType
  ) {
    await restartMdns({
      mode: serviceSettings.serviceType,
      host: settingsStore.serviceSettings.hostname,
    })
      .then(() => {
        settingsStore.serviceSettings.serviceType = serviceSettings.serviceType!;
      })
      .catch((e) => {
        console.error(e);
      });
  }
}

// 订阅 store 变化，持久化到本地
subscribe(settingsStore, async () => {
  await settingsLocalStore.set('settings', settingsStore);
});

export { initializeSettings, settingsStore, updateServiceSettings, updateSystemSettings };
