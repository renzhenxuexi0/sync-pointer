import { swapDevicePosition } from '@/store/devices';
import { settingsStore } from '@/store/settings';
import { DndContext, MouseSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Alert } from 'antd';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import DeviceGrid from './components/DeviceGrid';

function ScreenLayout() {
  const { t } = useTranslation();
  const serviceSettings = useSnapshot(settingsStore.serviceSettings);
  const mouseSensor = useSensor(MouseSensor);

  const sensors = useSensors(mouseSensor);

  return (
    <div
      className={`
        flex
        h-full
        flex-col
        items-center
        justify-center
        gap-4
      `}
    >
      {serviceSettings.serviceType === 'client' ? (
        <Alert
          message={t('screen-layout.client-alert')}
          type="warning"
          className={`h-6`}
          banner
        />
      ) : (
        <div className={`h-6`} />
      )}
      <div className={`mt-10`}>
        <DndContext
          sensors={sensors}
          onDragEnd={(e) => {
            if (serviceSettings.serviceType === 'client') {
              return;
            }
            if (e.over?.id) {
              swapDevicePosition(e.active.id as string, e.over.id as string);
            }
          }}
        >
          <DeviceGrid />
        </DndContext>
      </div>
    </div>
  );
}

export default ScreenLayout;
