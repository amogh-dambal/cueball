mod mqtt;

fn main() {
    println!("Hello, world!");
    println!("MQTT fixed header length (bytes): {}", mqtt::FIXED_HEADER_LEN_BYTES);
    let val: usize = mqtt::packet::new();
    println!("MQTT new packet: {}", val);
}
