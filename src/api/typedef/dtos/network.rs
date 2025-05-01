pub trait Packet {
    fn response(&self);
    fn json(&self);
    fn from_json();
}
