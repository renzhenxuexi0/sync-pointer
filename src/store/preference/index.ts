import { defineStore } from 'pinia';
export interface Preference {
    locale: 'en-US' | 'zh-CN';
    theme: 'light' | 'dark';
}

export const usePreferenceStore = defineStore(
    'preference',
    () => {
        const preference = ref({
            locale: 'zh-CN',
            theme: 'light',
        } as Preference);

        return {
            preference,
        };
    },
    {},
);
