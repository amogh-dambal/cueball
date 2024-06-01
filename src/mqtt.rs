// Exported modules
pub mod packet;
pub mod utils;

// Enums
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum QosLevel {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

// Functions