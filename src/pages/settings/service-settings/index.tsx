import { settingsStore, updateServiceSettings } from '@/store/settings';
import { ProFormRadio, ProFormText } from '@ant-design/pro-components';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import SettingsForm from '../components/SettingsForm';

function ServiceSettings() {
  const { t } = useTranslation();
  const serviceSettings = useSnapshot(settingsStore.serviceSettings);
  return (
    <SettingsForm
      initialValues={serviceSettings}
      onFinish={async (values, form) => {
        await updateServiceSettings(values);
        if (form) {
          form.setFieldsValue(serviceSettings);
        }
      }}
    >
      <ProFormText
        name="hostname"
        label={t('settings.service-settings.hostname.label')}
        allowClear={false}
        tooltip={t('settings.service-settings.hostname.tooltip')}
        rules={[
          {
            max: 15,
            message: t('settings.service-settings.hostname.max'),
          },
          {
            required: true,
            message: t('settings.service-settings.hostname.required'),
          },
          {
            pattern: /^[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?$/,
            message: t('settings.service-settings.hostname.tooltip'),
          },
        ]}
      />
      <ProFormRadio.Group
        name="serviceType"
        label={t('settings.service-settings.service-type.label')}
        options={[
          {
            label: t('settings.service-settings.service-type.server'),
            value: 'server',
          },
          {
            label: t('settings.service-settings.service-type.client'),
            value: 'client',
          },
        ]}
        radioType="button"
      />
    </SettingsForm>
  );
}

export default ServiceSettings;
