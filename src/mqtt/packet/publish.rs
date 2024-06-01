use std::mem;

use bytes::Buf;

use crate::mqtt::QosLevel;

use super::FixedHeader;

#[derive(Default)]
pub struct Publish {
    header: FixedHeader,
    packet_id: u16,
    topic: String,
    payload: String,
}

pub fn read_packet<R: Buf>(buf: &mut R, header: &FixedHeader) -> Publish {
    let mut p = Publish::default();
    p.header = *header;

    let remaining_len: usize = p.header.remaining_len as usize;
    let mut variable_header_len: usize = 0;

    // Read len of topic and then read topic
    let topic_len: u16 = buf.get_u16();
    let mut topic_data = vec![0u8; topic_len.into()];
    buf.copy_to_slice(&mut topic_data);

    p.topic = String::from_utf8(topic_data).expect("received invalid UTF-8 data for topic in PUBLISH packet");
    variable_header_len += mem::size_of::<u16>() + topic_len as usize;

    // Read packet ID
    if p.header.opts.qos().value() > QosLevel::AtMostOnce as u8{
        p.packet_id = buf.get_u16();
        variable_header_len += mem::size_of::<u16>();
    }
    
    // Read payload.
    let payload_len: usize = remaining_len - variable_header_len;
    let mut payload_data = vec![0u8; payload_len];
    buf.copy_to_slice(&mut payload_data);
    p.payload = String::from_utf8(payload_data).expect("invalid UTF-8 data in payload of PUBLISH packet");

    return p;
}

impl Publish {
    fn write<W: Buf>(&self, _buf: &mut W) {
        todo!()
    }
    
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io};
    
    use bytes::{BufMut, BytesMut};
    
    use crate::mqtt::{packet::{publish::read_packet, FixedHeader, Type}, QosLevel};

    #[test]
    fn test_read() {
        // Read test packet from file
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/packets/", "publish.bin");
        let mut f = File::open(path).expect("error opening file");
        let mut w = BytesMut::new().writer();
        io::copy(&mut f, &mut w).expect("error reading file");

        // Read packet header
        let mut buf = w.into_inner();
        let header = FixedHeader::read(&mut buf);

        // Read packet
        let packet = read_packet(&mut buf, &header);

        // Test that packet is correctly read
        assert_eq!(Type::PUBLISH as u8, packet.header.opts.packet_type().value());
        assert!(packet.header.opts.retain());
        assert_eq!(QosLevel::AtLeastOnce as u8, packet.header.opts.qos().value());
        assert_eq!(4, packet.topic.len());
        assert_eq!("info", packet.topic);
        assert_eq!(2, packet.packet_id);
        assert_eq!(6, packet.payload.len());
        assert_eq!("Cedalo", packet.payload);
    }
}