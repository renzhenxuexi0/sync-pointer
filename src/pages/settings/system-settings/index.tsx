import { systemSettingsStore, updateSystemSettings } from '@/store/settings/system';
import { ProFormSelect, ProFormSwitch } from '@ant-design/pro-components';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import SettingsForm from '../components/SettingsForm';

function SystemSettings() {
  const { t } = useTranslation();
  const systemSettings = useSnapshot(systemSettingsStore);
  return (
    <SettingsForm
      initialValues={systemSettings}
      onFinish={async (values) => {
        await updateSystemSettings(values);
      }}
    >
      <ProFormSelect
        name="locale"
        label={t('settings.system-settings.language.label')}
        valueEnum={{
          zhCN: t('settings.system-settings.language.zh-CN'),
          enUS: t('settings.system-settings.language.en-US'),
          auto: t('settings.system'),
        }}
        allowClear={false}
      />
      <ProFormSelect
        name="theme"
        label={t('settings.system-settings.theme.label')}
        valueEnum={{
          light: t('settings.system-settings.theme.light'),
          auto: t('settings.system'),
          dark: t('settings.system-settings.theme.dark'),
        }}
        allowClear={false}
      />
      <ProFormSwitch
        label={t('settings.system-settings.auto-start.label')}
        name={'autoStart'}
      ></ProFormSwitch>
    </SettingsForm>
  );
}

export default SystemSettings;
