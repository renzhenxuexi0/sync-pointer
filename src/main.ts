import '@/styles/tailwind/index.css';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';
import { createApp } from 'vue';
import App from './App.vue';
import { setupI18n } from './locales';
import router from './router';
import { setupStore } from './store';
import { usePreferenceStore } from './store/preference';

async function init() {
    // 创建
    const app = createApp(App);

    const pinia = createPinia();
    pinia.use(createPlugin());
    app.use(router);
    app.use(pinia);
    // 初始化存储
    await setupStore();
    const preferenceStore = usePreferenceStore();
    // 初始化i18n
    const i18n = setupI18n({
        locale: preferenceStore.preference.locale,
    });
    app.use(i18n);
    app.mount('#app');
}

init();
