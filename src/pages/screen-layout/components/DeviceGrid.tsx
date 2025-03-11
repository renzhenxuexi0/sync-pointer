import { createPositionKey, Device, devicesStore } from '@/store/devices';
import { useDroppable } from '@dnd-kit/core';
import { useSnapshot } from 'valtio';
import DeviceCell from './DeviceCell';

export interface GirdCell {
  row: number;
  col: number;
  disabled: boolean;
}

function GridCell(cell: GirdCell, device?: Device) {
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
        bg-slate-300
        sm:h-12
        sm:w-16
        md:h-18
        md:w-24
        lg:h-24
        lg:w-32
        dark:border-slate-700
        dark:bg-slate-900
        ${extendClassName}
      `}
    >
      {device ? <DeviceCell device={device} /> : <div></div>}
    </div>
  );
}

function DeviceGrid() {
  const state = useSnapshot(devicesStore.state);
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
        const device = state.devices[createPositionKey(row, col)];
        return GridCell(
          {
            row,
            col,
            // 如果只有一个设备那么哪里都可以移动
            disabled: !state.enableCells.has(index) && Object.keys(state.devices).length > 1,
          },
          device,
        );
      })}
    </div>
  );
}

export default DeviceGrid;
