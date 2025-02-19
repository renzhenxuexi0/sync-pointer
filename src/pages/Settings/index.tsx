import { settingsStore, updateServiceSettings, updateSystemSettings } from '@/store/settings';
import { Col, Form, Input, Radio, Row, Spin, Switch } from 'antd';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import SettingsCard from './components/SettingsCard.tsx';

function Settings() {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const preference = useSnapshot(settingsStore);

  return (
    <div className="h-[calc(100vh-60px)]">
      <Spin
        spinning={loading}
        fullscreen
      />
      <Form.Provider>
        <Row gutter={[16, 16]}>
          <Col span={12}>
            <SettingsCard
              title={t('settings.system-settings.label')}
              onFinish={async (values) => {
                setLoading(true);
                await updateSystemSettings(values)
                  .catch((reason) => {
                    console.log('Failed to update system settings' + reason);
                  })
                  .finally(() => {
                    setLoading(false);
                  });
              }}
              initialValues={preference.systemSettings}
            >
              {/* 语言设置 */}
              <Form.Item
                label={t('settings.system-settings.language.label')}
                name="locale"
              >
                <Radio.Group
                  block
                  options={[
                    { label: t('settings.system-settings.language.zh'), value: 'zh' },
                    { label: t('settings.system'), value: 'auto' },
                    { label: t('settings.system-settings.language.en'), value: 'en' },
                  ]}
                  optionType="button"
                />
              </Form.Item>
              {/* 主题设置 */}
              <Form.Item
                label={t('settings.system-settings.theme.label')}
                name="theme"
              >
                <Radio.Group
                  block
                  options={[
                    { label: t('settings.system-settings.theme.light'), value: 'light' },
                    { label: t('settings.system'), value: 'auto' },
                    { label: t('settings.system-settings.theme.dark'), value: 'dark' },
                  ]}
                  optionType="button"
                />
              </Form.Item>
              {/* 自动启动 */}
              <Form.Item
                label={t('settings.system-settings.auto-start.label')}
                name="autoStart"
              >
                <Switch size="default" />
              </Form.Item>
            </SettingsCard>
          </Col>
          <Col span={12}>
            <SettingsCard
              title={t('settings.service-settings.label')}
              onFinish={(values) => {
                setLoading(true);
                updateServiceSettings(values);
                setLoading(false);
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
