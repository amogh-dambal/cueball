use bytes::Buf;

#[derive(Debug, PartialEq, Eq)]
pub struct Topic {
    name: String,
    qos: u8,
}

impl Topic {
    pub fn new<R: Buf>(buf: &mut R) -> Self {
        let len = buf.get_u16();
        let mut data = vec![0u8; len.into()];
        buf.copy_to_slice(&mut data);
        
        let name = String::from_utf8(data).expect("invalid UTF-8 for topic name in SUBSCRIBE packet");
        let qos = buf.get_u8();

        Self::new_from_params(name, qos)
    }

    pub fn new_from_params(name: String, qos: u8) -> Self {
        Topic { name: name, qos: qos }
    }
}