use std::io::{Read, Write};

use tokio_util::bytes::{BufMut, BytesMut};
use tungstenite::Message;

use crate::api::typedef::BackendError;

pub fn read_prefixed_string(reader: &mut (impl Read + Unpin)) -> Result<String, BackendError> {
    let mut len_buf = [0u8; 2];
    reader.read_exact(&mut len_buf)?;
    let len = u16::from_be_bytes(len_buf);
    let mut res = vec![0u8; len as usize];

    reader.read_exact(&mut res)?;

    Ok(String::from_utf8(res)?)
}

pub fn write_prefixed_string(s: &str, writer: &mut (impl Write + Unpin)) -> Result<(), BackendError> {
    const MAX_LEN: usize = 1024;

    let bytes = s.as_bytes();
    if bytes.len() > MAX_LEN { return Err("String is way too large (1024+ bytes)".into()); }

    writer.write_all(&(bytes.len() as u16).to_be_bytes())?;
    writer.write_all(bytes)?;

    Ok(())
}

pub fn write_prefixed_string_bytebuf(s: &str, writer: &mut BytesMut) {
    let bytes = s.as_bytes();

    writer.put_slice(&(bytes.len() as u16).to_be_bytes());
    writer.put_slice(bytes);
}

pub fn encode_msg(source: &str, reader: &mut (impl Read + Unpin)) -> Result<Message, BackendError> {
    let mut bytebuf = BytesMut::new();

    write_prefixed_string_bytebuf(source, &mut bytebuf);

    let mut buf = [0u8; 2048];
    while let rd = reader.read(&mut buf)? && rd > 0 {
        bytebuf.put_slice(&mut buf[..rd]);
    }

    Ok(Message::Binary(bytebuf.freeze()))
}
