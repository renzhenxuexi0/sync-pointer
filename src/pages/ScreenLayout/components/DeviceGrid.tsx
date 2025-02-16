import { useDroppable } from '@dnd-kit/core';
import DeviceCell, { DeviceCellProps } from './DeviceCell';

export interface GirdCell {
  id: number;
  isDropDisabled: boolean;
}

export interface DeviceGridProps {
  cells: (GirdCell | DeviceCellProps)[];
  selectedDevice?: DeviceCellProps;
}

function gridCell(cell: GirdCell | DeviceCellProps, isDropDisabled: boolean) {
  const { setNodeRef } = useDroppable({
    id: cell.id,
    disabled: isDropDisabled,
  });
  const isDeviceCellProps = () => {
    return (cell as DeviceCellProps).hostname !== undefined;
  };

  return (
    <div
      key={cell.id}
      id={cell.id.toString()}
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
        ${
          isDropDisabled
            ? `
              cursor-not-allowed
              opacity-50
            `
            : ''
        }
      `}
    >
      {isDeviceCellProps() ? <DeviceCell {...(cell as DeviceCellProps)} /> : <span>{cell.id}</span>}
    </div>
  );
}

function DeviceGrid({ cells, selectedDevice }: DeviceGridProps) {
  return (
    <div
      className={`
        grid
        grid-cols-5
      `}
    >
      {cells.map((cell) => {
        const row = Math.floor(cell.id / 5);
        const col = cell.id % 5;
        const selectedRow = selectedDevice ? Math.floor(selectedDevice.id / 5) : -1;
        const selectedCol = selectedDevice ? selectedDevice.id % 5 : -1;

        const isDropDisabled = !selectedDevice
          ? false
          : !(
              (row === selectedRow && Math.abs(col - selectedCol) === 1) ||
              (col === selectedCol && Math.abs(row - selectedRow) === 1)
            );

        return gridCell(cell, isDropDisabled);
      })}
    </div>
  );
}

export default DeviceGrid;
