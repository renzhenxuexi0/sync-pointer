import { createPositionKey, Device, devicesStore, enableCellsStore } from '@/store/devices';
import { useDroppable } from '@dnd-kit/core';
import { useSnapshot } from 'valtio';
import DeviceCell from './DeviceCell';

export interface GirdCell {
  row: number;
  col: number;
  disabled: boolean;
}

function gridCell(cell: GirdCell, device?: Device) {
  const id = `${cell.row}-${cell.col}`;
  const { setNodeRef } = useDroppable({
    id,
    disabled: cell.disabled,
  });

  const extendClassName = cell.disabled ? 'cursor-not-allowed opacity-50' : '';

  return (
    <div
      key={id}
      id={id}
      ref={setNodeRef}
      className={`
        flex
        items-center
        justify-center
        border
        border-slate-200
        bg-slate-100
        dark:border-slate-700
        dark:bg-slate-900
        lg:h-24
        lg:w-32
        md:h-18
        md:w-24
        sm:h-12
        sm:w-16
        ${extendClassName}
      `}
    >
      {device ? <DeviceCell device={device} /> : <div></div>}
    </div>
  );
}

function DeviceGrid() {
  const devices = useSnapshot(devicesStore);
  const enableCells = useSnapshot(enableCellsStore);
  return (
    <div
      className={`
        grid
        grid-cols-5
      `}
    >
      {Array.from({ length: 25 }).map((_, index) => {
        const row = Math.floor(index / 5);
        const col = index % 5;
        const device = devices.get(createPositionKey(row, col));
        return gridCell(
          {
            row,
            col,
            disabled: !enableCells.has(index),
          },
          device,
        );
      })}
    </div>
  );
}

export default DeviceGrid;
