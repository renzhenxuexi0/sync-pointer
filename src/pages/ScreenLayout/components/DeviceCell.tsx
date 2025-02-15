import { useDraggable } from '@dnd-kit/core';

export interface Device {
  hostname: string;
  ip: string;
  port: number;
  serviceType: 'server' | 'client';
  position: {
    row: number;
    col: number;
  };
  status: 'online' | 'offline';
}

function DeviceCell(device: Device) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: device.hostname,
  });
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined;
  return (
    <div
      ref={setNodeRef}
      {...attributes}
      {...listeners}
      style={style}
      className={`
        flex
        size-12
        flex-row
        items-center
        justify-center
        gap-2
      `}
    >
      {/* 红绿点 */}
      <div
        className={`
          h-2
          w-2
          rounded-full
          ${device.status === 'online' ? 'bg-green-500' : 'bg-red-500'}
        `}
      ></div>
      <span className="text-xs">{device.hostname}</span>
    </div>
  );
}

export default DeviceCell;
