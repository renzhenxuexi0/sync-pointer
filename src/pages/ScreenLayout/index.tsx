import { preferenceStore } from '@/store/preference';
import { DndContext, MouseSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Alert } from 'antd';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import { Device } from './components/DeviceCell';
import DeviceGrid from './components/DeviceGrid';

function ScreenLayout() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);
  const mouseSensor = useSensor(MouseSensor);

  const sensors = useSensors(mouseSensor);

  const devices: Device[] = [
    {
      hostname: 'wuhy',
      serviceType: 'server',
      ip: '',
      port: 0,
      position: {
        row: 2,
        col: 2,
      },
      status: 'online',
    },
  ];

  return (
    <div className="flex h-full flex-col items-center justify-center gap-4">
      {preference.serviceSettings.serviceType === 'client' ? (
        <Alert
          message={t('screen-layout.client-alert')}
          type="warning"
          className="h-8"
          banner
        />
      ) : (
        <div className="h-8" />
      )}
      <DndContext
        sensors={sensors}
        onDragStart={() => {}}
        onDragEnd={() => {}}
        onDragCancel={() => {}}
      >
        <DeviceGrid devices={devices}></DeviceGrid>
      </DndContext>
    </div>
  );
}

export default ScreenLayout;
