use arbitrary_int::{u2, u4};
use bitbybit::bitfield;
use bytes::Buf;

#[bitfield(u8)]
#[derive(Default)]
pub struct FixedHeaderControlOptions {
    #[bits(4..=7, r)]
    packet_type: u4,

    #[bit(3, r)]
    dup: bool,

    #[bits(1..=2, r)]
    qos: u2,

    #[bit(0, r)]
    retain: bool,
}

// Structs
#[derive(Clone, Copy, Default)]
pub struct FixedHeader {
    pub remaining_len: u64,
    pub opts: FixedHeaderControlOptions,
}

impl FixedHeader {
    pub fn read<R: Buf>(buf: &mut R) -> Self {
        // Read the information about control packet type + additional flags.
        let control_data = buf.get_u8();
        let opts: FixedHeaderControlOptions = FixedHeaderControlOptions{
            raw_value: control_data,
        };
        
        // Read and decode the "remaining length" information.
        let remaining_len = Self::decode_remaining_length(buf);
        
        FixedHeader{
            opts: opts,
            remaining_len: remaining_len,
        }
    }

    fn decode_remaining_length<R: Buf>(buf: &mut R) -> u64 {
        let mut multiplier: u32 = 1;
        let mut result: u64 = 0;
        let mut done: bool = false;
        while !done {
            let eb: u8 = buf.get_u8();
            result += ((eb & 127) as u64) * (multiplier as u64);
    
            if multiplier > u32::pow(128, 3) {
                panic!("invalid value for remaining length")
            }
            multiplier *= 128;
            done = (eb & 128) == 0;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};
    use hex;

    use super::FixedHeader;
    use crate::mqtt::packet::Type;

    #[test]
    fn test_decode_remaining_length() {
        let mut buf = BytesMut::new();
        let mut v = hex::decode("1e").expect("unable to decode hex string");
        buf.put_slice(&mut v);

        assert_eq!(30, FixedHeader::decode_remaining_length::<BytesMut>(&mut buf));
    }

    #[test]
    fn test_read_fixed_header() {
        let mut buf = BytesMut::new();
        let mut v = hex::decode("101e").expect("unable to decode hex string");
        buf.put_slice(&mut v);

        let fh = FixedHeader::read(&mut buf);
        assert_eq!(Type::CONNECT as u8, fh.opts.packet_type().value());
        assert!(!fh.opts.retain());
        assert!(!fh.opts.dup());
        assert_eq!(0, fh.opts.qos().value());
    }
}