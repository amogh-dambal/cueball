use bytes::Buf;

use super::FixedHeader;

#[derive(Default)]
pub struct Ack {
    header: FixedHeader,
    packet_id: u16,
}

// All of these types of packets have the same structure as a 
// general ACK and don't really carry any additional data, so 
// we could use the same type if we really wanted to. This just
// gives us better separation of concerns.
pub type PubAck = Ack;
pub type PubRec = Ack;
pub type PubRel = Ack;
pub type PubComp = Ack;
pub type UnsubAck = Ack;
pub type PingReq = Ack;
pub type PingResp = Ack;
pub type Disconnect = Ack;

pub fn read_packet<R: Buf>(buf: &mut R, header: &FixedHeader) -> Ack {
    let mut p = Ack::default();
    p.header = *header;
    p.packet_id = buf.get_u16();
    return p;
}

impl Ack {
    fn write<W: Buf>(&self, buf: &mut W) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read() {

    }
}