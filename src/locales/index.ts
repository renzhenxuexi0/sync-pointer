import { createI18n, type I18n } from 'vue-i18n';

export type locale = 'en-US' | 'zh-CN';
export const SUPPORT_LOCALES = ['en-US', 'zh-CN'];

export function setupI18n(options = { locale: 'zh-CN' }) {
    const i18n = createI18n({
        legacy: false,
        locale: options.locale,
    }) as I18n;
    setI18nLanguage(i18n, options.locale as locale);
    loadLocaleMessages(i18n, options.locale as locale, 'default');
    return i18n;
}

/**
 * 设置语言
 * @param i18n i18n 实例
 * @param locale 语言
 */
export function setI18nLanguage(i18n: I18n, locale: locale) {
    if (i18n.global.locale instanceof String) {
        i18n.global.locale = locale;
    } else if (i18n.global.locale instanceof Object) {
        i18n.global.locale.value = locale;
    }
    /**
     * 注意：
     * 如果需要为请求头指定语言设置，例如 `fetch` API，请在此处设置。
     * 以下是 axios 的示例。
     *
     * axios.defaults.headers.common['Accept-Language'] = locale
     */
    document.querySelector('html')?.setAttribute('lang', locale);
}

export async function loadLocaleMessages(
    i18n: I18n,
    locale: locale,
    name: 'default' | 'home' | 'setting',
) {
    // load locale messages with dynamic import
    const messages = await import(`./langs/${locale}/${name}.json`);

    // set locale and locale message
    i18n.global.setLocaleMessage(locale, messages.default);

    return nextTick();
}
