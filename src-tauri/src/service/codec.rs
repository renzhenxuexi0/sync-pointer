use std::{marker::PhantomData, ops::Deref};

use anyhow::anyhow;
use futures_util::stream::{SplitSink, SplitStream};
use rkyv::{
    Archive, Archived,
    rancor::{BoxedError, Error as RancorError},
    ser::allocator::Arena,
    validation::{
        Validator, archive::ArchiveValidator, shared::SharedValidator,
    },
};
use tokio::net::TcpStream;
use tokio_util::codec;
use tokio_util::codec::Framed;
use tokio_util::{
    bytes::{Bytes, BytesMut},
    codec::LengthDelimitedCodec,
};

use super::protocols::base::DataPacket;

pub struct DataPacketCodec {
    inner: LengthDelimitedCodec,
    arena: Arena,
    _marker: PhantomData<DataPacket>,
}

impl Default for DataPacketCodec {
    fn default() -> Self {
        Self {
            inner: LengthDelimitedCodec::builder()
                .big_endian()
                .max_frame_length(u32::MAX as usize)
                .length_field_type::<u32>()
                .new_codec(),
            arena: Arena::new(),
            _marker: PhantomData,
        }
    }
}

impl From<LengthDelimitedCodec> for DataPacketCodec {
    fn from(inner: LengthDelimitedCodec) -> Self {
        Self { inner, arena: Arena::new(), _marker: PhantomData }
    }
}

// 添加 CheckedArchive 类型，用于安全地访问已验证的归档数据
pub struct CheckedArchive<T> {
    _marker: PhantomData<T>,
    bytes: Bytes,
}

impl<T> Deref for CheckedArchive<T>
where
    T: Archive,
{
    type Target = Archived<T>;

    fn deref(&self) -> &Self::Target {
        // SAFETY: 这是安全的，因为在创建 CheckedArchive 时已经进行了有效性验证
        unsafe { rkyv::access_unchecked(&self.bytes) }
    }
}

impl codec::Decoder for DataPacketCodec {
    // 修改返回类型为 CheckedArchive<DataPacket>
    type Item = CheckedArchive<DataPacket>;
    type Error = anyhow::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        // 将字节缓冲区转换为引用计数形式，并创建 CheckedArchive
        let bytes = match self.inner.decode(src)? {
            Some(bytes) => bytes.freeze(),
            None => return Ok(None),
        };

        // 校验数据
        rkyv::api::check_pos_with_context::<
            Archived<DataPacket>,
            _,
            RancorError,
        >(
            &bytes,
            rkyv::api::root_position::<Archived<DataPacket>>(bytes.len()),
            &mut Validator::new(
                ArchiveValidator::new(&bytes),
                SharedValidator::new(),
            ),
        )
        .map_err(|e| anyhow!("DataPacketDecoderError: {}", e))?;

        Ok(Some(CheckedArchive { _marker: PhantomData, bytes }))
    }
}

impl codec::Encoder<DataPacket> for DataPacketCodec {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: DataPacket,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let writer =
            rkyv::api::high::to_bytes_in_with_alloc::<_, _, BoxedError>(
                &item,
                Vec::with_capacity(size_of::<Archived<DataPacket>>()),
                self.arena.acquire(),
            )
            .map_err(|e| anyhow!("DataPacketEncoderError: {}", e))?;

        self.arena.shrink();

        self.inner
            .encode(writer.into(), dst)
            .map_err(|e| anyhow!("DataPacketEncoderError: {}", e))
    }
}

pub type DataPacketWriter =
    SplitSink<Framed<TcpStream, DataPacketCodec>, DataPacket>;
pub type DataPacketReader = SplitStream<Framed<TcpStream, DataPacketCodec>>;
