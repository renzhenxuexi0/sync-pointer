use crate::service::module::protocol::DataPacket;
use anyhow::anyhow;
use futures_util::stream::{SplitSink, SplitStream};
use rkyv::rancor::Error;
use tokio::net::TcpStream;
use tokio_util::bytes::{BufMut, BytesMut};
use tokio_util::codec;
use tokio_util::codec::Framed;

pub struct DataPacketCodec;

impl DataPacketCodec {
    const HEADER_LEN: usize = 4;
    /// 最多传送1G数据 对应头部 4字节 表示数据长度
    const MAX_SIZE: usize = 2 << 30;
}

impl codec::Encoder<DataPacket> for DataPacketCodec {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: DataPacket,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let data = rkyv::to_bytes::<Error>(&item)
            .map_err(|e| anyhow!("DataPacketCodecError: {}", e))?;
        let data = data.as_slice();
        let data_len = data.len();
        if data_len > Self::MAX_SIZE {
            return Err(anyhow!(
                "DataPacketCodecError: DataPacket size is too large"
            ));
        }
        // 最大传输u32的数据(可最多512G)，
        // 表示数据长度的u32数值占用4个字节
        dst.reserve(data.len() + Self::HEADER_LEN);

        // 先将长度值写入dst，即帧首，
        // 写入的字节序是大端的u32，读取时也要大端格式读取，
        // 也有小端的方法`put_u32_le()`，读取时也得小端读取
        dst.put_u32(data_len as u32);

        // 将数据写入dst
        dst.put(data);
        Ok(())
    }
}

impl codec::Decoder for DataPacketCodec {
    type Item = DataPacket;
    type Error = anyhow::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let buf_len = src.len();

        // 如果buf中的数据量连长度声明的大小都不足，则先跳过等待后面更多数据的到来
        if buf_len < 4 {
            return Ok(None);
        }

        // 先读取帧首，获得声明的帧中实际数据大小
        let mut length_bytes = [0u8; Self::HEADER_LEN];
        length_bytes.copy_from_slice(&src[..Self::HEADER_LEN]);
        let data_len = u32::from_be_bytes(length_bytes) as usize;
        if data_len > Self::MAX_SIZE {
            return Err(anyhow!(
                "DataPacketCodecError: DataPacket size is too large"
            ));
        }

        // 帧的总长度为 4 + frame_len
        let frame_len = data_len + Self::HEADER_LEN;

        // buf中数据量不够，跳过，并预先申请足够的空闲空间来存放该帧后续到来的数据
        if buf_len < frame_len {
            src.reserve(frame_len - buf_len);
            return Ok(None);
        }

        // 数据量足够了，从buf中取出数据转编成帧，并转换为指定类型后返回
        // 需同时将buf截断(split_to会截断)
        let frame_bytes = src.split_to(frame_len);
        let archived = rkyv::deserialize::<DataPacket, Error>(&frame_bytes[..])
            .map_err(|e| anyhow!("DataPacketCodecError: {}", e))?;

        Ok(Some(archived))
    }
}

pub type DataPacketWriter =
    SplitSink<Framed<TcpStream, DataPacketCodec>, DataPacket>;
pub type DataPacketReader = SplitStream<Framed<TcpStream, DataPacketCodec>>;
