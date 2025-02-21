import Connected from '@/assets/icon/connected.svg?react';
import Disconnected from '@/assets/icon/disconnected.svg?react';
import { Device } from '@/store/devices';
import { useDraggable } from '@dnd-kit/core';
import { Tooltip } from 'antd';
import { useTranslation } from 'react-i18next';

function DeviceCell({ device }: { device: Device }) {
  const id = `${device.row}-${device.col}`;
  const { t } = useTranslation();
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id,
  });
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined;
  return (
    <Tooltip
      placement="right"
      trigger="hover"
      title={
        <div>
          <div>
            {t('screen-layout.device-cell.hostname')}: {device.hostname}
          </div>
          <div>
            {t('screen-layout.device-cell.ip')}: {device.ip}
          </div>
          <div>
            {t('screen-layout.device-cell.port')}: {device.port}
          </div>
          <div>
            {t('screen-layout.device-cell.service-type')}: {device.serviceType}
          </div>
          <div>
            {t('screen-layout.device-cell.is-me')}:{' '}
            {device.isMe ? t('screen-layout.device-cell.yes') : t('screen-layout.device-cell.no')}
          </div>
          <div>row: {device.row}</div>
          <div>col: {device.col}</div>
        </div>
      }
    >
      <div
        ref={setNodeRef}
        {...attributes}
        {...listeners}
        style={{
          ...style,
          position: 'relative',
        }}
        className={`
          flex
          flex-row
          items-center
          justify-center
          gap-2
        `}
      >
        {device.status === 'online' ? (
          <Connected
            className={`
              cursor-grab
              lg:size-20
              md:size-12
              sm:size-10
            `}
          />
        ) : (
          <Disconnected
            className={`
              cursor-grab
              lg:size-20
              md:size-12
              sm:size-10
            `}
          />
        )}
      </div>
    </Tooltip>
  );
}

export default DeviceCell;
