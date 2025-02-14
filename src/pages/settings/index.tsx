import {
  preferenceStore,
  setPreferenceLocale,
  setPreferenceServiceType,
  setPreferenceTheme,
} from '@/store/preference';
import { Card, Col, Form, Radio, Row } from 'antd';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';

function Settings() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);
  return (
    <div>
      <Row gutter={[16, 16]}>
        <Col span={12}>
          <Card title={t('settings.system-settings')}>
            <Form
              labelAlign="left"
              labelCol={{ span: 8 }}
            >
              <Form.Item label={t('settings.service-type.label')}>
                {/* 服务类型单选 */}
                <Radio.Group
                  block
                  defaultValue={preference.serviceType}
                  options={[
                    { label: t('settings.service-type.server'), value: 'server' },
                    { label: t('settings.service-type.client'), value: 'client' },
                  ]}
                  optionType="button"
                  onChange={(e) => {
                    setPreferenceServiceType(e.target.value);
                  }}
                />
              </Form.Item>
              <Form.Item label={t('settings.language.label')}>
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
              </Form.Item>
              <Form.Item label={t('settings.theme.label')}>
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
              </Form.Item>
            </Form>
          </Card>
        </Col>
        <Col span={12}></Col>
      </Row>
    </div>
  );
}

export default Settings;
