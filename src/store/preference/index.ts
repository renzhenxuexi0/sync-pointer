import { setI18nLanguage } from '@/locales';
import { setTheme } from '@tauri-apps/api/app';
import { defineStore } from 'pinia';

export interface Preference {
    locale: 'en-US' | 'zh-CN';
    theme: 'light' | 'dark' | undefined;
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
            theme: 'light' | 'dark' | undefined,
        ) => {
            preference.value.theme = theme;
            await setTheme(theme);
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
