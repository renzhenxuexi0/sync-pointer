import { preferenceStore } from '@/store/preference';
import { DndContext, MouseSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Alert } from 'antd';
import { useState } from 'react';
import 'react-grid-layout/css/styles.css';
import { useTranslation } from 'react-i18next';
import 'react-resizable/css/styles.css';
import { useSnapshot } from 'valtio';
import { Device } from './components/DeviceCell';
import DeviceGrid from './components/DeviceGrid';

function ScreenLayout() {
  const { t } = useTranslation();
  const preference = useSnapshot(preferenceStore);
  const mouseSensor = useSensor(MouseSensor);

  const sensors = useSensors(mouseSensor);
  const [isDragging, setIsDragging] = useState(false);

  const devices: Device[] = [
    {
      hostname: 'hostname1',
      ip: '',
      port: 0,
      position: {
        row: 0,
        col: 0,
      },
      status: 'online',
    },
    {
      hostname: 'hostname2',
      ip: '',
      port: 0,
      position: {
        row: 0,
        col: 1,
      },
      status: 'offline',
    },
    {
      hostname: 'hostname3',
      ip: '',
      port: 0,
      position: {
        row: 1,
        col: 0,
      },
      status: 'online',
    },
    {
      hostname: 'hostname4',
      ip: '',
      port: 0,
      position: {
        row: 1,
        col: 1,
      },
      status: 'offline',
    },
  ];

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
      <DndContext
        sensors={sensors}
        onDragStart={() => setIsDragging(true)}
        onDragEnd={({ over }) => {
          setIsDragging(false);
        }}
        onDragCancel={() => setIsDragging(false)}
      >
        <DeviceGrid devices={devices}></DeviceGrid>
      </DndContext>
    </div>
  );
}

export default ScreenLayout;
