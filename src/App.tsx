import logo from '@/assets/logo.png';
import { AppstoreOutlined, GithubFilled, SettingOutlined } from '@ant-design/icons';
import { PageContainer, ProLayout } from '@ant-design/pro-components';
import { openUrl } from '@tauri-apps/plugin-opener';
import { Avatar, ConfigProvider, Spin } from 'antd';
import { ThemeProvider } from 'antd-style';
import enUS from 'antd/lib/locale/en_US';
import zhCN from 'antd/lib/locale/zh_CN';
import { lazy, Suspense, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Navigate, NavLink, Route, Routes } from 'react-router';
import { useSnapshot } from 'valtio';
import './App.css';
import NetworkSettings from './pages/settings/network-settings';
import { systemSettingsStore } from './store/settings/system';

// 懒加载路由组件
const ScreenLayout = lazy(() => import('@/pages/screen-layout'));
const SystemSettings = lazy(() => import('@/pages/settings/system-settings'));

// 加载提示组件
const LoadingComponent = () => (
  <div className="flex h-full w-full items-center justify-center">
    <Spin size="large" />
  </div>
);

function App() {
  const { t } = useTranslation();
  const [location, setLocation] = useState('/');
  const systemSettings = useSnapshot(systemSettingsStore);

  return (
    <ConfigProvider locale={systemSettings.locale === 'zh-CN' ? zhCN : enUS}>
      <ThemeProvider
        themeMode={systemSettings.theme}
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
          className={`
            h-screen
            w-screen
          `}
          logo={
            <Avatar
              alt={t('app.title')}
              shape="square"
              size="large"
              src={
                <img
                  src={logo}
                  alt="avatar"
                />
              }
            />
          }
          route={{
            children: [
              {
                path: '/screen-layout',
                name: t('menu.screen-layout'),
                icon: <AppstoreOutlined className="!text-[#08c]" />,
              },
              {
                path: '/settings',
                name: t('menu.settings'),
                icon: <SettingOutlined className="!text-[#08c]" />,
                children: [
                  {
                    path: '/settings/system',
                    name: t('menu.system-settings'),
                  },
                  {
                    path: '/settings/network',
                    name: t('menu.network-settings'),
                  },
                ],
              },
            ],
          }}
          location={{ pathname: location }}
          actionsRender={() => {
            return [
              <GithubFilled
                key="GithubFilled"
                onClick={async () => await openUrl('https://github.com/renzhenxuexi0/sync-pointer')}
              />,
            ];
          }}
          menuItemRender={(item, dom) => {
            return (
              <NavLink
                to={item.path ?? '/screen-layout'}
                onClick={() => setLocation(item.path ?? '/screen-layout')}
              >
                {dom}
              </NavLink>
            );
          }}
          defaultCollapsed
        >
          {/* 内容区 */}
          <PageContainer
            content={
              <Suspense fallback={<LoadingComponent />}>
                <Routes>
                  <Route
                    path="/"
                    element={
                      <Navigate
                        to="/screen-layout"
                        replace
                      />
                    }
                  />
                  <Route
                    path="screen-layout"
                    element={<ScreenLayout />}
                  ></Route>
                  <Route path="settings">
                    <Route
                      path="system"
                      element={<SystemSettings />}
                    />
                    <Route path='network' element={<NetworkSettings/>}/>
                  </Route>
                </Routes>
              </Suspense>
            }
            header={{
              title: undefined,
              breadcrumb: {},
            }}
            ghost
          ></PageContainer>
        </ProLayout>
      </ThemeProvider>
    </ConfigProvider>
  );
}

export default App;
