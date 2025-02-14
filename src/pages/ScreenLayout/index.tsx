import { preferenceStore } from '@/store/preference';
import { Alert } from 'antd';
import 'react-grid-layout/css/styles.css';
import { useTranslation } from 'react-i18next';
import 'react-resizable/css/styles.css';
import { useSnapshot } from 'valtio';

function ScreenLayout() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);

  return (
    <div className="flex h-full flex-col items-center justify-center">
      {preference.serviceType === 'client' ? (
        <Alert
          message={t('screen-layout.client-alert')}
          type="warning"
          className="h-8"
          banner
        />
      ) : null}
    </div>
  );
}

export default ScreenLayout;
