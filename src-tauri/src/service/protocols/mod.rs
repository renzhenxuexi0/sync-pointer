pub mod base;
pub mod clipboard;
pub mod connection;
pub mod device;
pub mod input;
pub mod system;

#[cfg(test)]
mod tests {
    use rkyv::{api::high::to_bytes_with_alloc, rancor::Error};
    use rkyv::{deserialize, ser::allocator::Arena};

    use crate::service::protocols::{
        base::{
            self, ArchivedDataPacket, DataPacket, PacketData, Request,
            TransportMode,
        },
        input::{self, KeyModifiers, MouseRequest},
    };

    #[test]
    fn test_input_broadcast() {
        let mouse_move = MouseRequest::Move {
            x: 100,
            y: 200,
            modifiers: KeyModifiers {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            },
        };

        let packet = DataPacket {
            message_id: None,
            transport_mode: TransportMode::Broadcast,
            timestamp: 1234567890,
            data: PacketData::Request(Request::Mouse(mouse_move)),
        };

        // 使用Arena进行序列化
        let mut arena = Arena::new();
        let bytes =
            to_bytes_with_alloc::<_, Error>(&packet, arena.acquire()).unwrap();

        // 使用安全API进行反序列化
        let archived =
            rkyv::access::<ArchivedDataPacket, Error>(&bytes[..]).unwrap();
        // 打印反序列化后的数据
        println!("timestamp: {}", archived.timestamp);

        // 完整反序列化
        let deserialized: base::DataPacket =
            deserialize::<_, Error>(archived).unwrap();
        assert_eq!(deserialized, packet);

        // 验证内容
        if let base::PacketData::Request(base::Request::Mouse(
            input::MouseRequest::Move { x, y, .. },
        )) = deserialized.data
        {
            assert_eq!(x, 100);
            assert_eq!(y, 200);
        } else {
            panic!("Wrong packet type");
        }
    }
}
