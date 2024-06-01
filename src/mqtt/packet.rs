use arbitrary_int::{u7, Number};
use bitbybit::bitfield;
use bytes::Buf;
use fixed_header::FixedHeader;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

mod ack;
mod connect;
mod fixed_header;
mod publish;
mod subscribe;
mod unsubscribe;

// Constants
const PROTOCOL_VERSION_FIELD_SIZE_BYTES: usize = 1;
const PROTOCOL_NAME_FIELD_SIZE_BYTES: usize = 6;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Type {
    CONNECT     = 1,
    CONNACK     = 2,
    PUBLISH     = 3,
    PUBACK      = 4,
    PUBREC      = 5,
    PUBREL      = 6,
    PUBCOMP     = 7,
    SUBSCRIBE   = 8,
    SUBACK      = 9,
    UNSUBSCRIBE = 10,
    UNSUBACK    = 11,
    PINGREQ     = 12,
    PINGRESP    = 13,
    DISCONNECT  = 14
}

// specialized ACK packets.
#[bitfield(u8)]
struct ConnAckFlags {
    #[bits(1..=7, r)]
    reserved: u7,
    
    #[bit(0, r)]
    session_present: bool,
}

pub struct ConnAck {
    header: FixedHeader,
    flags: ConnAckFlags,
}

pub struct SubAck {
    header: FixedHeader,
    packet_id: u16,
    return_codes: Vec<u16>,
}

pub enum Packet {
    Connect(connect::Connect),
    ConnAck(ConnAck),
    Publish(publish::Publish),
    PubAck(ack::PubAck),
    PubRec(ack::PubRec),
    PubRel(ack::PubRel),
    PubComp(ack::PubComp),
    Subscribe(subscribe::Subscribe),
    SubAck(SubAck),
    Unsubscribe(unsubscribe::Unsubscribe),
    UnsubAck(ack::UnsubAck),
    PingReq(ack::PingReq),
    PingResp(ack::PingResp),
    Disconnect(ack::Disconnect),
}

pub fn new<R: Buf>(buf: &mut R) -> Packet {
    let fixed_header = FixedHeader::read(buf);
    match Type::from_u8(fixed_header.opts.packet_type().value()).expect("Invalid packet type!") {
        Type::CONNECT => Packet::Connect(connect::read_packet(buf, &fixed_header)),
        Type::PUBLISH => Packet::Publish(publish::read_packet(buf, &fixed_header)),
        Type::SUBSCRIBE => Packet::Subscribe(subscribe::read_packet(buf, &fixed_header)),
        Type::UNSUBSCRIBE => Packet::Unsubscribe(unsubscribe::read_packet(buf, &fixed_header)),
        Type::PUBACK => Packet::PubAck(ack::read_packet(buf, &fixed_header)),
        Type::PUBREC => Packet::PubRec(ack::read_packet(buf, &fixed_header)),
        Type::PUBREL => Packet::PubRel(ack::read_packet(buf, &fixed_header)),
        Type::PUBCOMP => Packet::PubComp(ack::read_packet(buf, &fixed_header)),
        Type::UNSUBACK => Packet::UnsubAck(ack::read_packet(buf, &fixed_header)),
        Type::PINGREQ | Type::PINGRESP | Type::DISCONNECT => todo!("These packets are only one byte in length"),
        Type::CONNACK | Type::SUBACK => todo!("Handle these ACK's separately?"),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io, path::PathBuf};

    use bytes::{BufMut, BytesMut};

    use super::new;

    #[test]
    fn test_new() {
        let packet_files = vec!["connect.bin", "publish.bin", "subscribe.bin", "unsubscribe.bin"];
        for packet_file in packet_files {
            let mut path = PathBuf::new();
            path.push(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/packets/"));
            path.push(packet_file);

            let mut reader = File::open(path).expect("error opening file");
            let mut writer = BytesMut::new().writer();
            io::copy(&mut reader, &mut writer).expect("error reading file");
            let mut buf = writer.into_inner();

            let packet = new(&mut buf);
            match packet_file {
                "connect.bin" => {
                
                },
                "publish.bin" => {

                },
                "subscribe.bin" => {

                },
                "unsubscribe.bin" => {

                },
                _ => panic!("invalid test file")
            }
        }
    }
}