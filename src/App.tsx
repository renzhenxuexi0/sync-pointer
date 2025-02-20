import { AppstoreOutlined, GithubFilled, SettingOutlined } from '@ant-design/icons';
import { PageContainer, ProLayout } from '@ant-design/pro-components';
import { Avatar, ConfigProvider } from 'antd';
import { ThemeProvider } from 'antd-style';
import enUS from 'antd/lib/locale/en_US';
import zhCN from 'antd/lib/locale/zh_CN';
import { useTranslation } from 'react-i18next';
import { Navigate, NavLink, Route, Routes } from 'react-router';
import { useSnapshot } from 'valtio';
import './App.css';
import ScreenLayout from './pages/screen-layout';
import SystemSettings from './pages/settings/system-settings';
import { settingsStore } from './store/settings';

function App() {
  const { t } = useTranslation();
  const systemSettings = useSnapshot(settingsStore.systemSettings);

  return (
    <ConfigProvider locale={systemSettings.locale === 'zh' ? zhCN : enUS}>
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
                  src={'icon.png'}
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
                ],
              },
            ],
          }}
          actionsRender={() => {
            return [<GithubFilled key="GithubFilled" />];
          }}
          menuItemRender={(item, dom) => {
            return (
              <NavLink
                to={item.path ?? '/screen-layout'}
                onClick={() => item.path ?? '/screen-layout'}
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
                  ></Route>
                </Route>
              </Routes>
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
