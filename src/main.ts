import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';
import { createApp } from 'vue';
import { createVuetify } from 'vuetify';
import 'vuetify/styles';
import App from './App.vue';
import { aliases, custom } from './iconsets/custom';
import { i18n, setI18nLanguage } from './locales';
import router from './router';
import { setupStore } from './store';
import { usePreferenceStore } from './store/preference';
import './styles/index.css';

async function init() {
    // 创建
    const app = createApp(App);
    const pinia = createPinia();
    pinia.use(createPlugin());
    app.use(pinia);
    // 初始化存储
    await setupStore();
    const preferenceStore = usePreferenceStore();
    // 设置语言
    setI18nLanguage(preferenceStore.preference.locale);
    await preferenceStore.setPreferenceTheme(preferenceStore.preference.theme);
    app.use(i18n);
    const vuetify = createVuetify({
        icons: {
            defaultSet: 'custom',
            aliases,
            sets: {
                custom,
            },
        },
    });
    app.use(vuetify);
    app.use(router);
    app.mount('#app');
}

init();
