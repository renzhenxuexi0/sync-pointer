/* eslint-disable react/react-in-jsx-scope */
import { ConfigProvider, DatePicker, Radio, RadioChangeEvent, theme } from 'antd';
import enUS from 'antd/lib/locale/en_US';
import zhCN from 'antd/lib/locale/zh_CN';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import './App.css';
import { preferenceStore, setPreferenceLocale, setPreferenceTheme } from './store/preference';

function App() {
  const { t, i18n } = useTranslation();
  const preference = useSnapshot(preferenceStore.state);
  const changeLocale = (e: RadioChangeEvent) => {
    setPreferenceLocale(e.target.value);
    i18n.changeLanguage(e.target.value);
  };
  const changeTheme = (e: RadioChangeEvent) => {
    setPreferenceTheme(e.target.value);
  };
  return (
    <ConfigProvider
      locale={preference.locale === 'zh' ? zhCN : enUS}
      theme={{
        algorithm: preference.theme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
      }}
    >
      <main className="container">
        <header lang={preference.locale}></header>
        <h1>{t('app.loading')}</h1>
        <div style={{ marginBottom: 16 }}>
          <span style={{ marginInlineEnd: 16 }}>Change locale of components:</span>
          <Radio.Group
            value={preference.locale}
            onChange={changeLocale}
          >
            <Radio.Button
              key="en"
              value={'en'}
            >
              English
            </Radio.Button>
            <Radio.Button
              key="zh"
              value={'zh'}
            >
              中文
            </Radio.Button>
          </Radio.Group>
        </div>
        <div style={{ marginBottom: 16 }}>
          <span style={{ marginInlineEnd: 16 }}>Change theme of components:</span>
          <Radio.Group
            value={preference.theme}
            onChange={changeTheme}
          >
            <Radio.Button
              key="light"
              value={'light'}
            >
              Light
            </Radio.Button>
            <Radio.Button
              key="dark"
              value={'dark'}
            >
              Dark
            </Radio.Button>
          </Radio.Group>
        </div>
        <DatePicker />
      </main>
    </ConfigProvider>
  );
}

export default App;
