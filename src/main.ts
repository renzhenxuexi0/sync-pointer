import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import { createPinia } from 'pinia';
import { createPlugin } from 'tauri-plugin-pinia';
import { createApp } from 'vue';
import App from './App.vue';
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
    app.use(ElementPlus);
    app.use(pinia);
    // 初始化存储
    await setupStore();
    const preferenceStore = usePreferenceStore();
    // 设置语言
    setI18nLanguage(preferenceStore.preference.locale);
    await preferenceStore.setPreferenceTheme(preferenceStore.preference.theme);

    app.use(i18n);
    app.use(router);
    app.mount('#app');
}

init();
