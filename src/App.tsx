import { AppstoreOutlined, GithubFilled, SettingOutlined } from '@ant-design/icons';
import { PageContainer, ProLayout } from '@ant-design/pro-components';
import { Avatar, ConfigProvider } from 'antd';
import { ThemeProvider } from 'antd-style';
import enUS from 'antd/lib/locale/en_US';
import zhCN from 'antd/lib/locale/zh_CN';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { NavLink, Route, Routes } from 'react-router';
import { useSnapshot } from 'valtio';
import './App.css';
import ScreenLayout from './pages/ScreenLayout';
import Settings from './pages/settings';
import { preferenceStore, setPreferenceLocale } from './store/preference';

function App() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);
  // 只运行一次初始化语言
  useEffect(() => {
    setPreferenceLocale(preference.locale);
  }, []);

  return (
    <ConfigProvider locale={preference.locale === 'zh' ? zhCN : enUS}>
      <ThemeProvider
        themeMode={preference.theme}
        theme={{
          components: {
            Layout: {
              triggerHeight: 24,
              triggerColor: '#fff',
              triggerBg: '#fff',
            },
          },
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
            children: [
              {
                path: '/ScreenLayout',
                name: t('menu.screen-layout'),
                icon: <AppstoreOutlined className="!text-[#08c]" />,
              },
              {
                path: '/Settings',
                name: t('menu.settings'),
                icon: <SettingOutlined className="!text-[#08c]" />,
              },
            ],
          }}
          location={{
            pathname: '/ScreenLayout',
          }}
          actionsRender={() => {
            return [<GithubFilled key="GithubFilled" />];
          }}
          menuItemRender={(item, dom) => <NavLink to={item.path ?? '/ScreenLayout'}>{dom}</NavLink>}
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
                <Route
                  path="Settings"
                  element={<Settings />}
                ></Route>
              </Routes>
            }
            header={{
              title: undefined,
            }}
          ></PageContainer>
        </ProLayout>
      </ThemeProvider>
    </ConfigProvider>
  );
}

export default App;
