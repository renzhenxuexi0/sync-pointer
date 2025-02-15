import { useDroppable } from '@dnd-kit/core';
import DeviceCell, { Device } from './DeviceCell';

interface DeviceGridProps {
  devices: Device[];
}

function gridCell(row: number, col: number, device: Device | undefined) {
  const id = `device-${row}-${col}`;
  const { setNodeRef, isOver } = useDroppable({
    id,
  });
  return (
    <div
      key={id}
      id={id}
      ref={setNodeRef}
      className="flex items-center justify-center border border-slate-200 bg-slate-100 sm:h-12 sm:w-16 md:h-18 md:w-24 lg:h-24 lg:w-32 dark:border-slate-700 dark:bg-slate-900"
    >
      {device && <DeviceCell {...device} />}
      {<span>{isOver}</span>}
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
        return gridCell(row, col, device);
      })}
    </div>
  );
}

export default DeviceGrid;
