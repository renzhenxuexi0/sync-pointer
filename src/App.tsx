/* eslint-disable react/react-in-jsx-scope */
import { BulbOutlined, SettingOutlined, TranslationOutlined } from '@ant-design/icons';
import { ConfigProvider, FloatButton, Layout, Menu, theme } from 'antd';
import enUS from 'antd/lib/locale/en_US';
import zhCN from 'antd/lib/locale/zh_CN';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import './App.css';
import { preferenceStore, setPreferenceLocale, setPreferenceTheme } from './store/preference';

const { Sider, Content } = Layout;

function App() {
  const { t, i18n } = useTranslation();
  const preference = useSnapshot(preferenceStore.state);

  useEffect(() => {
    i18n.changeLanguage(preference.locale);
  }, []);

  return (
    <ConfigProvider
      locale={preference.locale === 'zh' ? zhCN : enUS}
      theme={{
        algorithm: preference.theme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
      }}
    >
      <Layout className="h-full w-full">
        <Sider>
          <Menu
            className="h-full"
            mode="inline"
            defaultSelectedKeys={['1']}
            items={[
              {
                key: '1',
                label: t('menu.dashboard'),
              },
              {
                key: '2',
                label: t('menu.settings'),
              },
            ]}
          />
        </Sider>
        <Layout>
          <Content>
            <span>{t('app.loading')}</span>
            {/* <DatePicker /> */}
          </Content>
        </Layout>
        <FloatButton.Group
          trigger="click"
          type="default"
          style={{ insetInlineEnd: 24 }}
          icon={<SettingOutlined />}
        >
          <FloatButton
            icon={<TranslationOutlined />}
            tooltip={t('settings.language')}
            onClick={() => {
              const newLocale = preference.locale === 'zh' ? 'en' : 'zh';
              setPreferenceLocale(newLocale);
            }}
          />
          <FloatButton
            icon={<BulbOutlined />}
            tooltip={t('settings.theme')}
            onClick={() => {
              const newTheme = preference.theme === 'dark' ? 'light' : 'dark';
              setPreferenceTheme(newTheme);
            }}
          />
        </FloatButton.Group>
      </Layout>
    </ConfigProvider>
  );
}

export default App;
