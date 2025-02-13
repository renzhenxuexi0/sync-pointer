import { preferenceStore, setPreferenceLocale, setPreferenceTheme } from '@/store/preference';
import { Radio, Switch } from 'antd';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';

function Settings() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);
  return (
    <div>
      {/* 语言选择 */}
      <Radio.Group
        block
        defaultValue={preference.locale}
        options={[
          { label: t('settings.language.zh'), value: 'zh' },
          { label: t('settings.system'), value: 'system' },
          { label: t('settings.language.en'), value: 'en' },
        ]}
        optionType="button"
        onChange={(e) => {
          setPreferenceLocale(e.target.value);
        }}
      />
      {/* 主题选择 */}
      <Radio.Group
        block
        defaultValue={preference.theme}
        options={[
          { label: t('settings.theme.light'), value: 'light' },
          { label: t('settings.system'), value: 'system' },
          { label: t('settings.theme.dark'), value: 'dark' },
        ]}
        optionType="button"
        onChange={(e) => {
          setPreferenceTheme(e.target.value);
        }}
      />
      <Switch
        className="w-24"
        key="server-client-switch"
        checkedChildren={t('settings.server')}
        unCheckedChildren={t('settings.client')}
        checked={preference.serverEnabled}
        onChange={(checked) => {
          preferenceStore.serverEnabled = checked;
        }}
      />
    </div>
  );
}

export default Settings;
