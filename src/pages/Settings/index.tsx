import SettingsCard from './components/SettingsCard.tsx';
import { settingsStore, updateServiceSettings, updateSystemSettings } from '@/store/settings';
import { Col, Form, Input, Radio, Row } from 'antd';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';

function Settings() {
  const { t } = useTranslation();
  const preference = useSnapshot(settingsStore);

  return (
    <div className="h-[calc(100vh-60px)]">
      <Form.Provider>
        <Row gutter={[16, 16]}>
          <Col span={12}>
            <SettingsCard
              title={t('Settings.system-Settings.label')}
              onFinish={(values) => {
                updateSystemSettings(values);
              }}
              initialValues={preference.systemSettings}
            >
              <Form.Item
                label={t('Settings.system-Settings.language.label')}
                name="locale"
              >
                <Radio.Group
                  block
                  options={[
                    { label: t('Settings.system-Settings.language.zh'), value: 'zh' },
                    { label: t('Settings.system'), value: 'system' },
                    { label: t('Settings.system-Settings.language.en'), value: 'en' },
                  ]}
                  optionType="button"
                />
              </Form.Item>
              <Form.Item
                label={t('Settings.system-Settings.theme.label')}
                name="theme"
              >
                <Radio.Group
                  block
                  options={[
                    { label: t('Settings.system-Settings.theme.light'), value: 'light' },
                    { label: t('Settings.system'), value: 'system' },
                    { label: t('Settings.system-Settings.theme.dark'), value: 'dark' },
                  ]}
                  optionType="button"
                />
              </Form.Item>
            </SettingsCard>
          </Col>
          <Col span={12}>
            <SettingsCard
              title={t('Settings.service-Settings.label')}
              onFinish={(values) => {
                updateServiceSettings(values);
              }}
              initialValues={preference.serviceSettings}
            >
              <Form.Item
                label={t('Settings.service-Settings.service-type.label')}
                name="serviceType"
              >
                <Radio.Group
                  block
                  options={[
                    {
                      label: t('Settings.service-Settings.service-type.server'),
                      value: 'server',
                    },
                    {
                      label: t('Settings.service-Settings.service-type.client'),
                      value: 'client',
                    },
                  ]}
                  optionType="button"
                />
              </Form.Item>
              <Form.Item
                label={t('Settings.service-Settings.hostname.label')}
                tooltip={t('Settings.service-Settings.hostname.tooltip')}
                rules={[
                  {
                    pattern: /^[a-zA-Z0-9-]+$/,
                    message: t('Settings.service-Settings.hostname.tooltip'),
                  },
                  {
                    max: 15,
                    message: t('Settings.service-Settings.hostname.max'),
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
