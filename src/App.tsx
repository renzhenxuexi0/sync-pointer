import {
  AppstoreOutlined,
  GithubFilled,
  MoonOutlined,
  SettingOutlined,
  SunOutlined,
  TranslationOutlined,
} from '@ant-design/icons';
import { PageContainer, ProLayout } from '@ant-design/pro-components';
import { Avatar, ConfigProvider, FloatButton, Switch, theme } from 'antd';
import enUS from 'antd/lib/locale/en_US';
import zhCN from 'antd/lib/locale/zh_CN';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { NavLink, Route, Routes } from 'react-router';
import { useSnapshot } from 'valtio';
import './App.css';
import ScreenLayout from './pages/ScreenLayout';
import { preferenceStore, setPreferenceLocale, setPreferenceTheme } from './store/preference';

function App() {
  const { t, i18n } = useTranslation();
  const preference = useSnapshot(preferenceStore.state);
  // 只运行一次初始化语言
  useEffect(() => {
    i18n.changeLanguage(preference.locale);
  }, []);

  const renderSettingsButtons = () => (
    <FloatButton.Group
      trigger="click"
      type="default"
      style={{ insetInlineEnd: 24 }}
      icon={<SettingOutlined />}
    >
      <FloatButton
        icon={<TranslationOutlined />}
        tooltip={preference.locale === 'zh' ? t('settings.language.zh') : t('settings.language.en')}
        onClick={() => {
          const newLocale = preference.locale === 'zh' ? 'en' : 'zh';
          setPreferenceLocale(newLocale);
        }}
      />
      <FloatButton
        icon={preference.theme === 'dark' ? <MoonOutlined /> : <SunOutlined />}
        tooltip={preference.theme === 'dark' ? t('settings.theme.dark') : t('settings.theme.light')}
        onClick={() => {
          const newTheme = preference.theme === 'dark' ? 'light' : 'dark';
          setPreferenceTheme(newTheme);
        }}
      />
    </FloatButton.Group>
  );

  return (
    <ConfigProvider
      locale={preference.locale === 'zh' ? zhCN : enUS}
      theme={{
        components: {
          Layout: {
            triggerHeight: 24,
            triggerColor: '#fff',
            triggerBg: '#fff',
          },
        },
        algorithm: preference.theme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
      }}
    >
      {/* 整个布局 */}
      <ProLayout
        title={t('app.title')}
        logo={
          <Avatar
            alt={t('app.title')}
            shape="square"
            size="large"
            src={
              <img
                src={'icon.png'}
                alt="avatar"
              />
            }
          />
        }
        route={{
          path: '/ScreenLayout',
          routes: [
            {
              path: '/ScreenLayout',
              name: t('menu.screen-layout'),
              icon: <AppstoreOutlined className="!text-[#08c]" />,
            },
          ],
        }}
        items={[{ key: '1', title: 'Menu Item 1' }]}
        actionsRender={() => {
          return [<GithubFilled key="GithubFilled" />];
        }}
        menuItemRender={(item, dom) => <NavLink to={item.path ?? '/'}>{dom}</NavLink>}
        defaultCollapsed
      >
        {/* 内容区 */}
        <PageContainer
          content={
            <Routes>
              <Route
                path="ScreenLayout"
                element={<ScreenLayout />}
              ></Route>
            </Routes>
          }
          header={{
            title: undefined,
            extra: [
              // server|client 开关
              <Switch
                key="server-client-switch"
                checkedChildren={t('settings.server')}
                unCheckedChildren={t('settings.client')}
                checked={preference.serverEnabled}
                onChange={(checked) => {
                  preferenceStore.state.serverEnabled = checked;
                }}
              />,
            ],
          }}
        ></PageContainer>
        {renderSettingsButtons()}
      </ProLayout>
    </ConfigProvider>
  );
}

export default App;
