import SettingsCard from '@/pages/settings/components/SettingsCard';
import { preferenceStore, updateServiceSettings, updateSystemSettings } from '@/store/preference';
import { Col, Form, Input, Radio, Row } from 'antd';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';

function Settings() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);

  return (
    <div className="h-[calc(100vh-60px)]">
      <Form.Provider>
        <Row gutter={[16, 16]}>
          <Col span={12}>
            <SettingsCard
              title={t('settings.system-settings.label')}
              onFinish={(values) => {
                updateSystemSettings(values);
              }}
              initialValues={preference.systemSettings}
            >
              <Form.Item
                label={t('settings.system-settings.language.label')}
                name="locale"
              >
                <Radio.Group
                  block
                  options={[
                    { label: t('settings.system-settings.language.zh'), value: 'zh' },
                    { label: t('settings.system'), value: 'system' },
                    { label: t('settings.system-settings.language.en'), value: 'en' },
                  ]}
                  optionType="button"
                />
              </Form.Item>
              <Form.Item
                label={t('settings.system-settings.theme.label')}
                name="theme"
              >
                <Radio.Group
                  block
                  options={[
                    { label: t('settings.system-settings.theme.light'), value: 'light' },
                    { label: t('settings.system'), value: 'system' },
                    { label: t('settings.system-settings.theme.dark'), value: 'dark' },
                  ]}
                  optionType="button"
                />
              </Form.Item>
            </SettingsCard>
          </Col>
          <Col span={12}>
            <SettingsCard
              title={t('settings.service-settings.label')}
              onFinish={(values) => {
                updateServiceSettings(values);
              }}
              initialValues={preference.serviceSettings}
            >
              <Form.Item
                label={t('settings.service-settings.service-type.label')}
                name="serviceType"
              >
                <Radio.Group
                  block
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
                  optionType="button"
                />
              </Form.Item>
              <Form.Item
                label={t('settings.service-settings.hostname.label')}
                tooltip={t('settings.service-settings.hostname.tooltip')}
                rules={[
                  {
                    pattern: /^[a-zA-Z0-9-]+$/,
                    message: t('settings.service-settings.hostname.tooltip'),
                  },
                  {
                    max: 15,
                    message: t('settings.service-settings.hostname.max'),
                  },
                ]}
                name="hostname"
              >
                <Input />
              </Form.Item>
            </SettingsCard>
          </Col>
        </Row>
      </Form.Provider>
    </div>
  );
}

export default Settings;
