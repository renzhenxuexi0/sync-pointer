import Icon from '@ant-design/icons';
import { useDraggable } from '@dnd-kit/core';

export interface Device {
  hostname: string;
  ip: string;
  port: number;
  position: {
    row: number;
    col: number;
  };
  status: 'online' | 'offline';
}

const DeviceConnectSvg = () => (
  <svg
    className="icon"
    viewBox="0 0 1024 1024"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    width="200"
    height="200"
  >
    <path
      d="M548.352 724.992v73.216h109.568V1024H365.568v-225.792h109.568v-73.216H146.432C65.536 724.992 0 659.456 0 579.072V146.432C0 65.536 65.536 0 145.92 0h731.648C958.464 0 1024 65.536 1024 145.92V578.56c0 80.896-65.536 146.432-145.92 146.432h-329.728z m402.432-549.376c0-56.32-39.424-102.4-87.552-102.4H160.768c-48.64 0-87.552 46.08-87.552 102.4v373.76c0 56.32 39.424 102.4 87.552 102.4h701.952c48.64 0 87.552-46.08 87.552-102.4l0.512-373.76z m0 767.488h-219.648v-73.216h219.648s36.352 0 36.352 36.352c0.512 36.864-36.352 36.864-36.352 36.864z m-877.568 0h219.648v-73.216H73.216s-36.352 0-36.352 36.352c-0.512 36.864 36.352 36.864 36.352 36.864z m764.928-706.56l-329.216 325.12-223.232-219.136 57.856-56.832 165.376 162.304 271.36-268.288 57.856 56.832z"
      fill="#35B030"
    ></path>
  </svg>
);

const DeviceDisconnectSvg = () => (
  <svg
    className="icon"
    viewBox="0 0 1024 1024"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    width="200"
    height="200"
  >
    <path
      d="M877.568 724.992h-329.216v73.216h109.568V1024H365.568v-225.792h109.568v-73.216H146.432C65.536 724.992 0 659.456 0 579.072V146.432C0 65.536 65.536 0 145.92 0h731.648C958.464 0 1024 65.536 1024 145.92V578.56c0 80.896-65.536 146.432-146.432 146.432 0.512 0 0 0 0 0z m-14.336-651.776H160.768c-48.64 0-87.552 46.08-87.552 102.4v373.76c0 56.32 39.424 102.4 87.552 102.4h701.952c48.64 0 87.552-46.08 87.552-102.4v-373.76c0.512-56.832-38.912-102.4-87.04-102.4z m-197.12 518.656L508.928 437.76l-165.376 162.304-57.856-56.832 165.376-162.304-165.376-162.304 57.856-56.832 165.376 162.304 157.184-154.112 57.856 56.832-157.184 154.112 157.184 154.112-57.856 56.832zM73.216 869.888h219.648v73.216H73.216s-36.352 0-36.352-36.352c-0.512-36.864 36.352-36.864 36.352-36.864z m914.432 36.352c0 36.352-36.352 36.352-36.352 36.352h-219.648v-73.216h219.648c-0.512 0.512 36.352 0.512 36.352 36.864z"
      fill="#C55252"
    ></path>
  </svg>
);

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
      className="flex size-12 flex-col items-center justify-center"
    >
      {device.status === 'online' ? (
        <Icon
          component={DeviceConnectSvg}
          className="size-8"
        />
      ) : (
        <Icon
          component={DeviceDisconnectSvg}
          className="size-8"
        />
      )}
      <span className="text-xs">{device.hostname}</span>
    </div>
  );
}

export default DeviceCell;
