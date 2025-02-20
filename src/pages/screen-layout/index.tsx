import { Device } from '@/store/devices';
import { settingsStore } from '@/store/settings';
import { DndContext, MouseSensor, useSensor, useSensors } from '@dnd-kit/core';
import { Alert } from 'antd';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useSnapshot } from 'valtio';
import { DeviceCellProps } from './components/DeviceCell';
import DeviceGrid, { DeviceGridProps, GirdCellProps } from './components/DeviceGrid';

function initCells(devices: Device[], hostname: string): (GirdCellProps | DeviceCellProps)[] {
  return Array.from({ length: 25 }).map((_, index) => {
    const device = devices.find((device) => device.id === index);
    if (device) {
      return {
        ...device,
        isMe: device.hostname === hostname,
        status: 'online',
      };
    }
    return {
      id: index,
      index,
      isDropDisabled: false,
    };
  });
}

function ScreenLayout() {
  const { t } = useTranslation();
  const serviceSettings = useSnapshot(settingsStore.serviceSettings);
  const mouseSensor = useSensor(MouseSensor);

  const sensors = useSensors(mouseSensor);

  const [deviceGridProps, setDeviceGridProps] = useState<DeviceGridProps>({
    cells: initCells(
      [
        {
          hostname: serviceSettings.hostname,
          ip: '',
          id: 12,
          port: 0,
          serviceType: 'server',
        },
      ],
      serviceSettings.hostname,
    ),
    selectedDevice: undefined,
  });

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
            // console.log(e);
            if (serviceSettings.serviceType === 'client') {
              return;
            }
            if (e.over?.id) {
              const index = e.over.id as number;
              const deviceIndex = deviceGridProps.cells.find(
                (device): device is DeviceCellProps =>
                  (device as DeviceCellProps).hostname === e.active.id,
              )?.id;
              if (deviceIndex) {
                // 交换两个cell的位置
                const cells = [...deviceGridProps.cells];
                [cells[index], cells[deviceIndex]] = [cells[deviceIndex], cells[index]];

                setDeviceGridProps({
                  ...deviceGridProps,
                  cells: cells,
                });
              }
            }
          }}
        >
          <DeviceGrid {...deviceGridProps}></DeviceGrid>
        </DndContext>
      </div>
    </div>
  );
}

export default ScreenLayout;
