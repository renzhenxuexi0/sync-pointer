import { usePreferenceStore } from './preference';

export async function setupStore() {
    const preferenceStore = usePreferenceStore();
    await preferenceStore.$tauri.start();
}
