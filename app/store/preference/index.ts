import { store } from 'tauri-plugin-valtio';

export interface Preference {
    theme: "light" | "dark" | "auto";
    locale: "zh-CN" | "en-US";
}

export const preferenceStore = store('preference', { 
    locale: "zh-CN",
    theme: "auto",
 });

 export function setPreferenceLocale(locale: Preference["locale"]) {
    preferenceStore.state.locale = locale;
 }