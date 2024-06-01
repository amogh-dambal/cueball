use bytes::Buf;

use crate::mqtt::utils::topic::Topic;

use super::FixedHeader;

#[derive(Default)]
pub struct Unsubscribe {
    header: FixedHeader,
    packet_id: u16,
    topics: Vec<Topic>
}

pub fn read_packet<R: Buf>(buf: &mut R, header: &FixedHeader) -> Unsubscribe {
    let mut p = Unsubscribe::default();
    p.header = *header;
    p.packet_id = buf.get_u16();
    
    while buf.has_remaining() {
        let topic = Topic::new(buf);
        p.topics.push(topic);
    }
    return p;
}

impl Unsubscribe {
   
    fn write<W: Buf>(&self, buf: &mut W) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io};

    use bytes::{BufMut, BytesMut};

    use crate::mqtt::{packet::{unsubscribe::read_packet, FixedHeader, Type}, utils::topic::Topic};

    #[test]
    fn test_read() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/packets/", "unsubscribe.bin");
        let mut reader = File::open(path).expect("error opening file");
        let mut writer = BytesMut::new().writer();
        io::copy(&mut reader, &mut writer).expect("error reading file");

        let mut buf = writer.into_inner();
        let header = FixedHeader::read(&mut buf);

        let packet = read_packet(&mut buf, &header);

        // Test packet
        assert_eq!(Type::UNSUBSCRIBE as u8, packet.header.opts.packet_type().value());
        assert_eq!(1, packet.packet_id);
        assert_eq!(1, packet.topics.len());
        assert_eq!(Topic::new_from_params("mytopic".to_string(), 1), packet.topics[0]);
    }

}