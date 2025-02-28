use crate::service::codec::DataPacketWriter;
use crate::service::module::connection::DeviceInfo;
use crate::service::module::protocol::DataPacket;
use crate::service::server::listener::ServerListener;
use futures_util::SinkExt;
use tokio::io::AsyncWriteExt;

pub struct SessionContext {
    device_info: Option<DeviceInfo>,
    server_listener: ServerListener,
    writer: DataPacketWriter,
}

impl SessionContext {
    pub fn new(
        writer: DataPacketWriter,
        server_listener: ServerListener,
    ) -> Self {
        SessionContext { device_info: None, writer, server_listener }
    }

    pub fn device_info(&self) -> Option<&DeviceInfo> {
        self.device_info.as_ref()
    }

    pub fn set_device_info(&mut self, device_info: DeviceInfo) {
        self.device_info = Some(device_info);
    }

    pub async fn send(&mut self, data: DataPacket) -> anyhow::Result<()> {
        self.writer.send(data).await?;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.writer.close().await?;
        self.server_listener.shutdown().await?;
        Ok(())
    }
}
