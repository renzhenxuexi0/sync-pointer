import { useDroppable } from '@dnd-kit/core';
import DeviceCell, { Device } from './DeviceCell';

interface DeviceGridProps {
  devices: Device[];
}

function gridCell(index: number, device: Device | undefined) {
  const { setNodeRef } = useDroppable({
    id: `device-${index}`,
  });
  return (
    <div
      key={index}
      ref={setNodeRef}
      className="flex h-24 w-30 items-center justify-center border border-slate-200 bg-slate-100 dark:border-slate-700 dark:bg-slate-600"
    >
      {device && <DeviceCell {...device} />}
    </div>
  );
}

function DeviceGrid({ devices }: DeviceGridProps) {
  return (
    <div className="grid grid-cols-5">
      {Array.from({ length: 25 }).map((_, index) => {
        const row = Math.floor(index / 5);
        const col = index % 5;
        const device = devices.find(
          (device) => device.position.row === row && device.position.col === col,
        );
        return gridCell(index, device);
      })}
    </div>
  );
}

export default DeviceGrid;
