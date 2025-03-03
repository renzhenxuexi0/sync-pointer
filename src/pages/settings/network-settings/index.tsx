import { networkSettingsStore, updateNetworkSettings } from '@/store/settings/network';
import { ProFormRadio, ProFormText } from '@ant-design/pro-components';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import SettingsForm from '../components/SettingsForm';

function NetworkSettings() {
  const { t } = useTranslation();
  const networkSettings = useSnapshot(networkSettingsStore.state);
  return (
    <SettingsForm
      initialValues={networkSettings}
      onFinish={async (values) => {
        await updateNetworkSettings(values);
      }}
    >
      <ProFormText
        name="hostname"
        label={t('settings.network-settings.hostname.label')}
        allowClear={false}
        tooltip={t('settings.network-settings.hostname.tooltip')}
        rules={[
          {
            max: 15,
            message: t('settings.network-settings.hostname.max'),
          },
          {
            required: true,
            message: t('settings.network-settings.hostname.required'),
          },
          {
            pattern: /^[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?$/,
            message: t('settings.network-settings.hostname.tooltip'),
          },
        ]}
      />
      <ProFormRadio.Group
        name="serviceType"
        label={t('settings.network-settings.service-type.label')}
        options={[
          {
            label: t('settings.network-settings.service-type.server'),
            value: 'server',
          },
          {
            label: t('settings.network-settings.service-type.client'),
            value: 'client',
          },
        ]}
        radioType="button"
      />
    </SettingsForm>
  );
}

export default NetworkSettings;
