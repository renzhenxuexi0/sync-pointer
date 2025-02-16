import { useDraggable } from '@dnd-kit/core';

export interface DeviceCellProps {
  id: number;
  hostname: string;
  ip: string;
  port: number;
  serviceType: 'server' | 'client';
  isMe: boolean;
  status: 'online' | 'offline';
}

function DeviceCell(props: DeviceCellProps) {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: props.hostname,
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
          ${props.status === 'online' ? 'bg-green-500' : 'bg-red-500'}
        `}
      ></div>
      <span
        className={`
          md:text-xs
          sm:text-xs
        `}
      >
        {props.hostname + (props.isMe ? '(you)' : '')}
      </span>
    </div>
  );
}

export default DeviceCell;
