import { error, info } from '@tauri-apps/plugin-log';
import { createI18n } from 'vue-i18n';
import enUS from './en-US/default.json';
import zhCN from './zh-CN/default.json';

export type locale = 'en-US' | 'zh-CN';
export const SUPPORT_LOCALES = ['en-US', 'zh-CN'];

export const i18n = createI18n({
    legacy: false,
    locale: 'zh-CN',
    messages: {
        'zh-CN': {
            default: zhCN,
        },
        'en-US': {
            default: enUS,
        },
    },
});

/**
 * 设置语言
 * @param locale 语言
 */
export function setI18nLanguage(locale: locale) {
    i18n.global.locale.value = locale;
    document.querySelector('html')?.setAttribute('lang', locale);
    info(`已设置语言为 ${locale}`);
}

export async function loadLocaleMessages(name: string | undefined) {
    // 懒加载语言包
    try {
        if (!name) {
            return;
        }
        const zhCNMessage = await import(`@/locales/zh-CN/${name}.json`);
        const enUSMessage = await import(`@/locales/en-US/${name}.json`);

        const allMessages = {
            'zh-CN': zhCNMessage.default,
            'en-US': enUSMessage.default,
        };

        for (const [locale, messages] of Object.entries(allMessages)) {
            i18n.global.mergeLocaleMessage(locale, { [name]: messages });
        }
        info(`已加载 ${name} 的语言包`);
    } catch (e) {
        error(`加载 ${name} 的语言包失败 error: ${JSON.stringify(e)}`);
    }

    return nextTick();
}
