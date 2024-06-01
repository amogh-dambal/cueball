use arbitrary_int::u2;
use bitbybit::bitfield;
use bytes::Buf;

use super::{FixedHeader, PROTOCOL_NAME_FIELD_SIZE_BYTES, PROTOCOL_VERSION_FIELD_SIZE_BYTES };

#[bitfield(u8)]
#[derive(Default)]
struct ConnectFlags {
    #[bit(7, r)]
    username: bool,

    #[bit(6, r)]
    password: bool,

    #[bit(5, r)]
    will_retain: bool,

    #[bits(3..=4, r)]
    will_qos: u2,

    #[bit(2, r)]
    will: bool,

    #[bit(1, r)]
    clean_session: bool,

    #[bit(0, r)]
    reserved: bool,
}

#[derive(Default)]
pub struct Connect {
    header: FixedHeader,
    flags: ConnectFlags,
    keepalive: u16,
    client_id: String,
    password: String,
    username: String,
    will_message: String,
    will_topic: String,
}

pub fn read_packet<R: Buf>(buf: &mut R, header: &FixedHeader) -> Connect {
    // header will be read by something else and passed to `new` since it's always the same
    let mut p = Connect::default();
    p.header = *header;

    // TODO: Implement checks on protocol name and reserved flags.
    // For now, skip those checks.
    buf.advance(PROTOCOL_NAME_FIELD_SIZE_BYTES);
    buf.advance(PROTOCOL_VERSION_FIELD_SIZE_BYTES);
    
    // read the connection flags.
    let flags = buf.get_u8();
    p.flags = ConnectFlags { raw_value: flags };

    // read the keepalive bits
    let keepalive = buf.get_u16();
    p.keepalive = keepalive;
    println!("keep-alive: {}", keepalive);

    // CID len
    let cid_len: u16 = buf.get_u16();
    if cid_len > 0 {
        println!("client id length: {}", cid_len);
        let mut client_id_buf = vec![0u8; cid_len.into()];
        buf.copy_to_slice(&mut client_id_buf);
        match String::from_utf8(client_id_buf) {
            Ok(val) => {
                p.client_id = val;
                println!("client id: {}", p.client_id);
            }
            Err(err) => {
                panic!("err: {}", err);
            }
        }
    }

    if p.flags.will() {
        println!("reading will topic + message\n");
        // read will topic + message
        // read topic
        let will_topic_len: u16 = buf.get_u16();
        let mut will_topic_buf = vec![0u8; will_topic_len.into()];
        buf.copy_to_slice(&mut will_topic_buf);
        
        match String::from_utf8(will_topic_buf) {
            Ok(val) => {
                p.will_topic = val;
            }
            Err(err) => panic!("err: {}", err),
        }

        // read message
        let will_message_len = buf.get_u16();
        let mut will_message_buf = vec![0u8; will_message_len.into()];
        buf.copy_to_slice(&mut will_message_buf);
        
        match String::from_utf8(will_message_buf) {
            Ok(val) => {
                p.will_message = val;
            }
            Err(err) => panic!("err: {}", err),
        }
    } else {
        println!("no will in CONNECT packet, not reading it.");
    }

    if p.flags.username() {
        let username_len = buf.get_u16();
        let mut username_buf = vec![0u8; username_len.into()];
        buf.copy_to_slice(&mut username_buf);
        
        match String::from_utf8(username_buf) {
            Ok(val) => {
                p.username = val;
                println!("username: {}", p.username);
            }
            Err(err) => panic!("err: {}", err),
        }
    }
    if p.flags.password() {
        let pass_len: u16 = buf.get_u16();
        let mut pass_buf = vec![0u8; pass_len.into()];
        buf.copy_to_slice(&mut pass_buf);
        
        match String::from_utf8(pass_buf) {
            Ok(val) => p.password = val,
            Err(err) => panic!("error converting password to string: {}", err)
        }
        println!("password: {}", p.password);
    }
    return p;
}

impl Connect {
    fn write<W: Buf>(&self, _buf: &mut W) {}
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io};
    use bytes::{BufMut, BytesMut};
    use crate::mqtt::packet::{connect::read_packet, FixedHeader};

    #[test]
    fn test_read() {
        // Read test packet from file
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/packets/", "connect.bin");
        let mut f = File::open(path).expect("error opening file");
        let mut w = BytesMut::new().writer();
        io::copy(&mut f, &mut w).expect("error reading file");

        // Read packet header
        let mut buf = w.into_inner();
        let header = FixedHeader::read(&mut buf);
    
        // Create packet from binary data
        let packet = read_packet(&mut buf, &header);

        // Assert packet is correctly read.
        assert_eq!(60, packet.keepalive);
        assert_eq!("myPy", packet.client_id);
        assert_eq!("client", packet.username);
        assert_eq!("pass", packet.password);
    }

    #[test]
    fn test_write() {}
}

