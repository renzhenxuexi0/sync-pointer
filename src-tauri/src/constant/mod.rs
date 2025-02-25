pub const MAIN_WINDOW_LABEL: &str = "main";
pub const TRAY_ICON_ID: &str = "main";
pub const MDNS_SERVICE_TYPE: &str = "_sp._udp.local.";
pub const MDNS_SERVER_NAME: &str = "sp";
// 默认服务端口
pub const DEFAULT_TCP_PORT: u16 = 3457;
// 默认UDP端口
pub const DEFAULT_UDP_PORT: u16 = 3458;
// 默认MDNS端口
pub const DEFAULT_MDNS_PORT: u16 = 3456;
// 客户端重试次数
pub const DEFAULT_CLIENT_RETRY_COUNT: u32 = 5;
// 连接超时时间（秒）
pub const DEFAULT_CONNECTION_TIMEOUT_SECONDS: u64 = 10;
// 心跳间隔（秒）
pub const DEFAULT_HEARTBEAT_INTERVAL_SECONDS: u64 = 5;
