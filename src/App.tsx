/* eslint-disable react/react-in-jsx-scope */
import { MoonOutlined, SettingOutlined, SunOutlined, TranslationOutlined } from '@ant-design/icons';
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

  // 只运行一次初始化语言
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
                label: t('menu.screen-layout'),
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
            tooltip={
              preference.locale === 'zh' ? t('settings.language.zh') : t('settings.language.en')
            }
            onClick={() => {
              const newLocale = preference.locale === 'zh' ? 'en' : 'zh';
              setPreferenceLocale(newLocale);
            }}
          />
          <FloatButton
            icon={preference.theme === 'dark' ? <MoonOutlined /> : <SunOutlined />}
            tooltip={
              preference.theme === 'dark' ? t('settings.theme.dark') : t('settings.theme.light')
            }
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
