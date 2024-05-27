use arbitrary_int::{u2, u4};
use bitbybit::bitfield;

// Exported modules
pub mod packet;

// Constants
pub const FIXED_HEADER_LEN_BYTES: usize = 2;

// Enums
pub enum QosLevel {
    AT_MOST_ONCE,
    AT_LEAST_ONCE,
    EXACTLY_ONCE,
}

// Structs
#[bitfield(u8)]
pub struct FixedHeader {
    // #[bits(8..=15, r)]
    // remaining_length: u8,

    #[bits(4..=7, r)]
    packet_type: u4,

    #[bit(3, r)]
    dup: bool,

    #[bits(1..=2, r)]
    qos: u2,

    #[bit(0, r)]
    retain: bool,
}
