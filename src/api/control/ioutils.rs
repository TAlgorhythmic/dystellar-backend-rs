use hyper::body::Buf;
use tokio_util::bytes::{BufMut, BytesMut};
use tungstenite::Message;

use crate::api::{routers::privileged::REGULAR_MESSAGE, typedef::BackendError};

pub fn read_prefixed_string(b: &mut tungstenite::Bytes) -> Result<String, BackendError> {
    let len = b.get_u16();
    let mut res = vec![0u8; len as usize];
    b.copy_to_slice(&mut res);

    Ok(String::from_utf8(res)?)
}

pub fn write_prefixed_string(s: &str, writer: &mut BytesMut) {
    let bytes = s.as_bytes();

    writer.put_slice(&(bytes.len() as u16).to_be_bytes());
    writer.put_slice(bytes);
}

pub fn encode_msg(source: &str, reader: &mut tungstenite::Bytes) -> Result<Message, BackendError> {
    let mut bytebuf = BytesMut::with_capacity(source.len() + 2 + reader.remaining());

    bytebuf.put_u8(REGULAR_MESSAGE);
    write_prefixed_string(source, &mut bytebuf);
    bytebuf.extend_from_slice(reader);

    Ok(Message::Binary(bytebuf.freeze()))
}
