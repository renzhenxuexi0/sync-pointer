import { setI18nLanguage } from '@/locales';
import { setTheme } from '@tauri-apps/api/app';
import { defineStore } from 'pinia';

export interface Preference {
    locale: 'en-US' | 'zh-CN';
    theme: 'light' | 'dark' | 'system';
}

export const usePreferenceStore = defineStore(
    'preference',
    () => {
        const preference = ref({
            locale: 'zh-CN',
            theme: 'light',
        } as Preference);

        const getPreferenceTheme = computed(
            () => preference.value.theme || 'system',
        );

        const setPreferenceLocale = (locale: 'en-US' | 'zh-CN') => {
            preference.value.locale = locale;
            setI18nLanguage(locale);
        };

        const setPreferenceTheme = async (
            theme: 'light' | 'dark' | 'system',
        ) => {
            preference.value.theme = theme;
            await setTheme(theme === 'system' ? undefined : theme);
        };

        return {
            preference,
            getPreferenceTheme,
            setPreferenceLocale,
            setPreferenceTheme,
        };
    },
    {
        tauri: {
            saveOnChange: true,
        },
    },
);
