use arbitrary_int::{u12, u13, u2, u7};
use bitbybit::bitfield;

use super::FixedHeader;

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

// TODO: Should have a serde_mqtt thing going on here so that
// we can efficiently (de)serialize these packet structures.
pub trait Packet {

}


#[bitfield(u8)]
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
    reserved: bool
}

pub struct Connect {
    header: FixedHeader,
    flags: ConnectFlags,
    keepalive: u16,
    client_id: &'static str,
    password: &'static str,
    username: &'static str,
    will_message: &'static str,
    will_topic: &'static str,
}

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

struct Topic {
    name: &'static str,
    qos: usize,
}

pub struct Subscribe {
    header: FixedHeader,
    packet_id: u16,
    topics: Vec<Topic>
}

pub struct SubAck {
    header: FixedHeader,
    packet_id: u16,
    return_codes: Vec<u16>,
}

pub struct Unsubscribe {
    header: FixedHeader,
    packet_id: u16,
    topics: Vec<Topic>
}

pub struct Publish {
    header: FixedHeader,
    packet_id: u16,
    topic_names: Vec<&'static str>,
    payloads: Vec<u8>,
}

pub struct Ack {
    header: FixedHeader,
    packet_id: u16,
}

// All of these types of packets have the same structure as a 
// general ACK and don't really carry any additional data, so 
// we could use the same type if we really wanted to. This just
// gives us better separation of concerns.
type PubAck = Ack;
type PubRec = Ack;
type PubRel = Ack;
type PubComp = Ack;
type UnsubAck = Ack;
type PingReq = Ack;
type PingResp = Ack;
type Disconnect = Ack;

pub fn new() -> usize {
    return 0;
}